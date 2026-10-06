//! The pre-McRegion ChunkLoader used by SaveFormatOld's SaveHandler.
use crate::{
    nbt::{self, Compound, Tag},
    storage::chunk_to_nbt,
    world::chunk::Chunk,
};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct LegacyChunkLoader181 {
    pub directory: PathBuf,
    pub create_if_necessary: bool,
}

fn base36(value: i32) -> String {
    let mut magnitude = (value as i64).unsigned_abs();
    let mut digits = Vec::new();
    loop {
        digits.push(b"0123456789abcdefghijklmnopqrstuvwxyz"[(magnitude % 36) as usize]);
        magnitude /= 36;
        if magnitude == 0 {
            break;
        }
    }
    if value < 0 {
        digits.push(b'-');
    }
    digits.reverse();
    String::from_utf8(digits).unwrap()
}

impl LegacyChunkLoader181 {
    pub fn new(directory: &Path, create_if_necessary: bool) -> Self {
        Self {
            directory: directory.into(),
            create_if_necessary,
        }
    }

    /// Each mkdir is attempted separately and its result is ignored, as in File.mkdir.
    pub fn chunk_file(&self, x: i32, z: i32) -> Option<PathBuf> {
        let mut path = self.directory.join(base36(x & 63));
        if !path.exists() {
            if !self.create_if_necessary {
                return None;
            }
            let _ = fs::create_dir(&path);
        }
        path = path.join(base36(z & 63));
        if !path.exists() {
            if !self.create_if_necessary {
                return None;
            }
            let _ = fs::create_dir(&path);
        }
        path = path.join(format!("c.{}.{}.dat", base36(x), base36(z)));
        if !path.exists() && !self.create_if_necessary {
            None
        } else {
            Some(path)
        }
    }

    pub fn read_root(&self, x: i32, z: i32) -> Option<Compound> {
        let path = self.chunk_file(x, z)?;
        if !path.exists() {
            return None;
        }
        crate::storage181::read_gzip_file(&path).ok()
    }

    /// Returns the physical SizeOnDisk change. The old file's length is removed
    /// before entering the catch block; a failed write/rename keeps that change.
    pub fn save_chunk(&self, chunk: &Chunk, world_time: i64) -> io::Result<i64> {
        let path = self.chunk_file(chunk.x, chunk.z).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "ChunkLoader.chunkFileForXZ returned null",
            )
        })?;
        let old_size = if path.exists() {
            fs::metadata(&path).map(|m| m.len() as i64).unwrap_or(0)
        } else {
            0
        };
        let temporary = self.directory.join("tmp_chunk.dat");
        let write = (|| -> io::Result<()> {
            // FileOutputStream is opened before storeChunkInCompound.
            let mut file = fs::File::create(&temporary)?;
            let mut root = Compound::new();
            root.put("Level", Tag::Compound(chunk_to_nbt(chunk, world_time)));
            use std::io::Write;
            file.write_all(&nbt::write_gzip(&root)?)?;
            drop(file);
            if path.exists() {
                if path.is_dir() {
                    let _ = fs::remove_dir(&path);
                } else {
                    let _ = fs::remove_file(&path);
                }
            }
            // Windows File.renameTo does not replace an existing destination.
            if !path.exists() {
                let _ = fs::rename(&temporary, &path);
            }
            Ok(())
        })();
        if write.is_err() {
            return Ok(-old_size);
        }
        let new_size = fs::metadata(&path).map(|m| m.len() as i64).unwrap_or(0);
        Ok(new_size - old_size)
    }

    pub fn tick(&self) {}
    pub fn save_extra_data(&self) {}
    pub fn save_extra_chunk_data(&self, _chunk: &Chunk) {}
}
