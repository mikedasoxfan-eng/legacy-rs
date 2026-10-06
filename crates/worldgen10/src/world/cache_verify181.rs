//! Original-bytecode callback replay for the alternate chunk cache.
use super::{
    chunk::Chunk,
    legacy_cache181::{CacheHost, ChunkHandle, LegacyChunkCache},
};
use crate::JavaRandom;
use std::{cell::RefCell, rc::Rc};
fn int(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_be_bytes());
}
fn text(o: &mut Vec<u8>, s: &str) {
    o.extend((s.len() as u16).to_be_bytes());
    o.extend(s.as_bytes());
}
struct Host {
    time: i64,
    spawn: bool,
    loader: bool,
    fail: bool,
    next: u64,
    events: Rc<RefCell<Vec<String>>>,
}
impl Host {
    fn event(&self, s: String) {
        self.events.borrow_mut().push(s);
    }
}
impl CacheHost for Host {
    fn world_time(&self) -> i64 {
        self.time
    }
    fn finding_spawn(&self) -> bool {
        self.spawn
    }
    fn has_loader(&self) -> bool {
        self.loader
    }
    fn has_generator(&self) -> bool {
        true
    }
    fn load(&mut self, x: i32, z: i32) -> Result<Option<ChunkHandle>, String> {
        self.event(format!("read:{x},{z}"));
        if self.fail && x & 3 == 0 {
            Err("fixture loader error".into())
        } else {
            Ok(None)
        }
    }
    fn generate(&mut self, x: i32, z: i32) -> ChunkHandle {
        self.event(format!("generate:{x},{z}"));
        self.next += 1;
        let mut c = Chunk::new(x, z, Box::new([0; 32768]));
        c.version = self.next;
        Rc::new(RefCell::new(c))
    }
    fn translate(&mut self, c: &ChunkHandle) {
        self.event(format!("translate:{}", c.borrow().version));
    }
    fn on_load(&mut self, c: &ChunkHandle) {
        if c.borrow().version != 0 {
            c.borrow_mut().loaded = true;
            self.event(format!("load:{}", c.borrow().version));
        }
    }
    fn on_unload(&mut self, c: &ChunkHandle) {
        if c.borrow().version != 0 {
            c.borrow_mut().loaded = false;
            self.event(format!("unload:{}", c.borrow().version));
        }
    }
    fn populate(&mut self, x: i32, z: i32) {
        self.event(format!("populate:{x},{z}"));
    }
    fn save(&mut self, c: &ChunkHandle) {
        let c = c.borrow();
        self.event(format!("save:{}:{}", c.version, c.last_save_time));
    }
    fn save_extra(&mut self, c: &ChunkHandle) {
        self.event(format!("extra:{}", c.borrow().version));
    }
    fn flush(&mut self) {
        self.event("flush".into());
    }
    fn tick_loader(&mut self) {
        self.event("loader-tick".into());
    }
    fn unload_generator(&mut self) -> bool {
        self.event("generator-tick".into());
        false
    }
}
fn chunk(out: &mut Vec<u8>, c: Option<&ChunkHandle>) {
    int(out, c.map_or(-1, |c| c.borrow().version as i32));
    if let Some(c) = c {
        let c = c.borrow();
        int(out, c.x);
        int(out, c.z);
        for b in [
            c.loaded,
            c.populated,
            c.modified,
            c.never_save,
            c.has_entities,
        ] {
            out.push(b as u8);
        }
        out.extend(c.last_save_time.to_be_bytes());
    }
}
fn dump(out: &mut Vec<u8>, p: &LegacyChunkCache, h: &Host) {
    int(out, p.last_x);
    int(out, p.last_z);
    chunk(out, p.last.as_ref());
    for (i, c) in p.chunks.iter().enumerate() {
        if c.is_some() {
            int(out, i as i32);
            chunk(out, c.as_ref());
        }
    }
    int(out, -1);
    let mut es = h.events.borrow_mut();
    int(out, es.len() as i32);
    for e in es.iter() {
        text(out, e);
    }
    es.clear();
    text(out, &p.make_string());
}
pub fn dump_cache(mode: &str, seed: i64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut r = JavaRandom::new(seed);
    let mut blank = Chunk::new(0, 0, Box::new([0; 32768]));
    blank.never_save = true;
    let mut p = LegacyChunkCache::new(Rc::new(RefCell::new(blank)));
    let mut h = Host {
        time: 0,
        spawn: false,
        loader: mode != "null",
        fail: mode == "failure",
        next: 0,
        events: Rc::new(RefCell::new(Vec::new())),
    };
    for x in -6..=6 {
        for z in -6..=6 {
            p.provide_chunk(&mut h, x, z);
        }
    }
    dump(&mut out, &p, &h);
    h.time = 1000;
    for c in p.chunks.iter().flatten() {
        if c.borrow().version != 0 {
            let mut c = c.borrow_mut();
            c.has_entities = r.next_bool();
            c.never_save = r.next_int(9) == 0;
            c.modified = r.next_bool();
        }
    }
    let es = h.events.clone();
    let mut progress = |n| es.borrow_mut().push(format!("progress:{n}"));
    for _ in 0..4 {
        out.push(p.save_chunks(&mut h, false, Some(&mut progress)) as u8);
        dump(&mut out, &p, &h);
    }
    out.push(p.save_chunks(&mut h, true, Some(&mut progress)) as u8);
    dump(&mut out, &p, &h);
    p.set_current_chunk_over(64, -64);
    chunk(&mut out, Some(&p.provide_chunk(&mut h, 6, 6)));
    out.push(p.chunk_exists(6, 6) as u8);
    dump(&mut out, &p, &h);
    for x in 58..=70 {
        for z in -70..=-58 {
            p.load_chunk(&mut h, x, z);
        }
    }
    dump(&mut out, &p, &h);
    h.spawn = true;
    for x in 96..=99 {
        for z in 96..=99 {
            p.provide_chunk(&mut h, x, z);
        }
    }
    h.spawn = false;
    dump(&mut out, &p, &h);
    p.set_current_chunk_over(-96, -96);
    for x in -98..=-94 {
        for z in -98..=-94 {
            p.provide_chunk(&mut h, x, z);
        }
    }
    dump(&mut out, &p, &h);
    for (x, z) in [
        (-111, -111),
        (-112, -112),
        (-81, -81),
        (-80, -80),
        (i32::MAX, i32::MIN),
    ] {
        out.push(p.can_chunk_exist(x, z) as u8);
        out.push(p.chunk_exists(x, z) as u8);
        chunk(&mut out, Some(&p.provide_chunk(&mut h, x, z)));
    }
    dump(&mut out, &p, &h);
    out.push(p.unload100_oldest_chunks(&mut h) as u8);
    out.push(p.can_save() as u8);
    dump(&mut out, &p, &h);
    out.extend(r.state().to_be_bytes());
    out
}
