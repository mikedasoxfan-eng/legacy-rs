//! RegionFileCache's process-wide sharing, 256-file limit and explicit flush.
use crate::storage::RegionFile;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

static REGIONS: OnceLock<Mutex<HashMap<PathBuf, RegionFile>>> = OnceLock::new();
fn cache() -> &'static Mutex<HashMap<PathBuf, RegionFile>> {
    REGIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Like the synchronized Java API, hold the cache lock for the operation.
/// A Storage drop does not close these files; flush and the capacity limit do.
pub fn with_region<T>(
    directory: &Path,
    cx: i32,
    cz: i32,
    op: impl FnOnce(&mut RegionFile) -> io::Result<T>,
) -> io::Result<T> {
    let folder = directory.join("region");
    let path = folder.join(format!("r.{}.{}.mcr", cx >> 5, cz >> 5));
    let mut regions = cache().lock().unwrap_or_else(|e| e.into_inner());
    if !regions.contains_key(&path) {
        if !folder.exists() {
            std::fs::create_dir_all(&folder)?;
        }
        if regions.len() >= 256 {
            regions.clear();
        }
        regions.insert(path.clone(), RegionFile::open(&path)?);
    }
    op(regions.get_mut(&path).unwrap())
}
pub fn clear() {
    cache().lock().unwrap_or_else(|e| e.into_inner()).clear();
}
pub fn size_delta(directory: &Path, cx: i32, cz: i32) -> io::Result<i32> {
    with_region(directory, cx, cz, |r| Ok(r.take_size_delta()))
}
pub fn read_chunk(directory: &Path, cx: i32, cz: i32) -> io::Result<Option<Vec<u8>>> {
    with_region(directory, cx, cz, |r| Ok(r.read_chunk(cx & 31, cz & 31)))
}
pub fn write_chunk(directory: &Path, cx: i32, cz: i32, compressed: &[u8]) -> io::Result<()> {
    with_region(directory, cx, cz, |r| {
        r.write_chunk(cx & 31, cz & 31, compressed)
    })
}
