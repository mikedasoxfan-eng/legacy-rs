//! ThreadedFileIOBase and ThreadedChunkLoader's synchronized queues.
use crate::nbt::{self, Compound};
use std::{collections::HashSet, io, path::{Path, PathBuf}, sync::{Arc, LazyLock, Mutex}, time::Duration};

pub trait IThreadedFileIO: Send + Sync {
    /// True after processing an entry; false only when called on an empty queue.
    fn write_next(&self) -> bool;
}
#[derive(Default)]
struct FileIoState {
    tasks: Vec<Arc<dyn IThreadedFileIO>>,
    queued: i64,
    completed: i64,
    flushing: bool,
}
pub struct ThreadedFileIOBase { state: Mutex<FileIoState> }
impl ThreadedFileIOBase {
    pub(crate) fn counters10(&self) -> (i64, i64) {
        let state = self.state.lock().unwrap();
        (state.queued, state.completed)
    }
    fn new() -> Arc<Self> {
        let io = Arc::new(Self { state: Mutex::new(FileIoState::default()) });
        let worker = io.clone();
        std::thread::Builder::new().name("File IO Thread".into()).spawn(move || loop { worker.process_pass(); })
            .expect("create File IO Thread");
        io
    }
    pub fn enqueue(&self, task: Arc<dyn IThreadedFileIO>) {
        let mut state = self.state.lock().unwrap();
        if !state.tasks.iter().any(|t| Arc::ptr_eq(t, &task)) {
            state.queued = state.queued.wrapping_add(1);
            state.tasks.push(task);
        }
    }
    fn process_pass(&self) {
        let mut i = 0;
        loop {
            let task = self.state.lock().unwrap().tasks.get(i).cloned();
            let Some(task) = task else { break; };
            let more = task.write_next();
            let mut state = self.state.lock().unwrap();
            if !more {
                state.tasks.remove(i);
                state.completed = state.completed.wrapping_add(1);
            } else { i += 1; }
            let flushing = state.flushing;
            drop(state);
            if flushing { std::thread::yield_now(); }
            else { std::thread::sleep(Duration::from_millis(10)); }
        }
        if self.state.lock().unwrap().tasks.is_empty() { std::thread::sleep(Duration::from_millis(25)); }
    }
    pub fn flush(&self) {
        self.state.lock().unwrap().flushing = true;
        loop {
            let state = self.state.lock().unwrap();
            if state.queued == state.completed { break; }
            drop(state);
            std::thread::sleep(Duration::from_millis(10));
        }
        self.state.lock().unwrap().flushing = false;
    }
}
pub static FILE_IO: LazyLock<Arc<ThreadedFileIOBase>> = LazyLock::new(ThreadedFileIOBase::new);

#[derive(Clone, Debug)]
pub struct ThreadedChunkLoaderPending {
    pub x: i32,
    pub z: i32,
    pub root: Compound,
}
#[derive(Default)]
pub struct PendingChunks {
    entries: Vec<ThreadedChunkLoaderPending>,
    coordinates: HashSet<(i32, i32)>,
}
impl PendingChunks {
    /// Return true if a new entry was appended, false if an existing entry was replaced.
    pub fn enqueue(&mut self, pending: ThreadedChunkLoaderPending) -> bool {
        let key = (pending.x, pending.z);
        if self.coordinates.contains(&key) {
            if let Some(entry) = self.entries.iter_mut().find(|e| (e.x, e.z) == key) {
                *entry = pending;
                return false;
            }
        }
        self.entries.push(pending);
        self.coordinates.insert(key);
        true
    }
    pub fn load(&self, x: i32, z: i32) -> Option<Compound> {
        if !self.coordinates.contains(&(x, z)) { return None; }
        self.entries.iter().find(|e| e.x == x && e.z == z).map(|e| e.root.clone())
    }
    pub fn pop(&mut self) -> Option<ThreadedChunkLoaderPending> {
        if self.entries.is_empty() { return None; }
        let pending = self.entries.remove(0);
        self.coordinates.remove(&(pending.x, pending.z));
        Some(pending)
    }
}
/// Oracle replays only: make each File IO write atomic with respect to chunk loads (see
/// `ThreadedChunkLoader::load_root`). Off in the game, which keeps the original's race.
pub static ATOMIC_IO_FOR_REPLAY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static IO_GATE: Mutex<()> = Mutex::new(());
/// Loads that coincided with the File IO thread writing that same chunk.
pub static LOADS_DURING_WRITE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
/// Of those, the loads that found neither the pending entry nor region data (regenerated).
pub static LOADS_MISSED_DURING_WRITE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn io_gate() -> Option<std::sync::MutexGuard<'static, ()>> {
    ATOMIC_IO_FOR_REPLAY.load(std::sync::atomic::Ordering::SeqCst).then(|| IO_GATE.lock().unwrap_or_else(|e| e.into_inner()))
}
pub struct ThreadedChunkLoader {
    pub directory: PathBuf,
    pending: Mutex<PendingChunks>,
    /// The entry `write_next` has taken off the list and is still writing.
    in_flight: Mutex<Option<(i32, i32)>>,
}
impl ThreadedChunkLoader {
    pub(crate) fn queued_snapshot10(&self) -> Vec<ThreadedChunkLoaderPending> {
        self.pending.lock().unwrap().entries.clone()
    }
    pub fn new(directory: &Path) -> Arc<Self> {
        Arc::new(Self { directory: directory.to_owned(), pending: Mutex::new(PendingChunks::default()), in_flight: Mutex::new(None) })
    }
    pub fn load_pending(&self, x: i32, z: i32) -> Option<Compound> { self.pending.lock().unwrap().load(x, z) }
    /// ThreadedChunkLoader.loadChunk up to the NBT read: the pending entry, else the region
    /// file (None when absent or unreadable). Like the original, the pending lookup and the
    /// file read are not atomic with respect to `write_next`: a load of the chunk the File IO
    /// thread has just taken off the list but not yet written sees neither, and the chunk is
    /// regenerated (or a stale copy read). Oracle replays pin that thread interleaving with
    /// `ATOMIC_IO_FOR_REPLAY`, as the server oracle does on the original classes.
    pub fn load_root(&self, x: i32, z: i32) -> Option<Compound> {
        let _gate = io_gate();
        let during = |l: &Self| *l.in_flight.lock().unwrap() == Some((x, z));
        let before = during(self);
        let root = match self.load_pending(x, z) {
            Some(root) => Some(root),
            None => crate::region_cache181::read_chunk(&self.directory, x, z).ok().flatten()
                .and_then(|raw| nbt::read_root(&mut &raw[..]).ok()),
        };
        if before || during(self) {
            LOADS_DURING_WRITE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if root.is_none() { LOADS_MISSED_DURING_WRITE.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
        }
        root
    }
    pub fn save(self: &Arc<Self>, x: i32, z: i32, root: Compound) {
        let mut pending = self.pending.lock().unwrap();
        if pending.enqueue(ThreadedChunkLoaderPending { x, z, root }) { FILE_IO.enqueue(self.clone()); }
    }
    pub fn write_pending(&self, pending: ThreadedChunkLoaderPending) -> io::Result<()> {
        let mut raw = Vec::new();
        nbt::write_root(&mut raw, &pending.root)?;
        crate::region_cache181::write_chunk(&self.directory, pending.x, pending.z, &crate::compression10::zlib(&raw)?)
    }
}
impl IThreadedFileIO for ThreadedChunkLoader {
    /// ThreadedChunkLoader.func_40324_c: the head entry leaves the list (and its coordinate
    /// set) inside the lock; it is serialized and written to the region file outside it.
    fn write_next(&self) -> bool {
        let _gate = io_gate();
        let pending = {
            let mut list = self.pending.lock().unwrap();
            let pending = list.pop();
            *self.in_flight.lock().unwrap() = pending.as_ref().map(|p| (p.x, p.z));
            pending
        };
        let Some(pending) = pending else { return false; };
        if let Err(error) = self.write_pending(pending) { eprintln!("saving queued chunk: {error}"); }
        *self.in_flight.lock().unwrap() = None;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn entry(x: i32, value: i32) -> ThreadedChunkLoaderPending {
        let mut root = Compound::new(); root.put("value", nbt::Tag::Int(value));
        ThreadedChunkLoaderPending { x, z: 0, root }
    }
    #[test]
    fn replacement_retains_fifo_position_and_pending_read_uses_latest_data() {
        let mut queue = PendingChunks::default();
        assert!(queue.enqueue(entry(1, 10)));
        assert!(queue.enqueue(entry(2, 20)));
        assert!(!queue.enqueue(entry(1, 30)));
        assert_eq!(queue.load(1, 0).unwrap().int("value"), 30);
        let first = queue.pop().unwrap();
        assert_eq!(first.x, 1);
        assert_eq!(first.root.int("value"), 30);
        assert!(queue.load(1, 0).is_none());
        assert_eq!(queue.pop().unwrap().x, 2);
        assert!(queue.pop().is_none());
    }
    #[test]
    fn singleton_flush_makes_the_latest_snapshots_readable_from_region_files() {
        let dir=std::env::current_dir().unwrap().join("../../target/r10-threaded-save-tests").join(
            format!("{}-{}",std::process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&dir).unwrap();
        let loader=ThreadedChunkLoader::new(&dir);
        loader.save(1,0,entry(1,10).root);loader.save(2,0,entry(2,20).root);loader.save(1,0,entry(1,30).root);
        FILE_IO.flush();
        for (x,value) in [(1,30),(2,20)] {
            let raw=crate::region_cache181::read_chunk(&dir,x,0).unwrap().unwrap();
            let actual=nbt::read_root(&mut &raw[..]).unwrap();assert_eq!(actual.int("value"),value);
            assert!(loader.load_pending(x,0).is_none());
        }
    }
}
