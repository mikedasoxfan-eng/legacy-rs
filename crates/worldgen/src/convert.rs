//! `SaveConverterMcRegion`: worlds saved by Alpha and early Beta (one gzip file per chunk
//! under base-36 folders, level.dat `version` 0) become McRegion worlds the first time they
//! are opened, exactly as the original converts them.

use crate::scenario::LoadStep;
use crate::storage::{RegionFile, Storage};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// `isOldMapFormat`: level.dat exists and its save version is 0.
pub fn is_old_map_format(dir: &Path) -> bool {
    Storage::read_level(dir).is_some_and(|l| l.version == 0)
}

/// A `c.<x>.<z>.dat` chunk file (`ChunkFile`).
struct ChunkFile {
    path: PathBuf,
    x: i32,
    z: i32,
}

/// `ChunkFolderPattern`: one or two characters of `[0-9a-z]`.
fn is_chunk_folder(p: &Path) -> bool {
    p.is_dir()
        && p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
            (1..=2).contains(&n.len()) && n.bytes().all(|b| b.is_ascii_digit() || b.is_ascii_lowercase())
        })
}

/// `ChunkFilePattern`: `c\.(-?[0-9a-z]+)\.(-?[0-9a-z]+)\.dat`, coordinates in base 36.
fn chunk_coords(name: &str) -> Option<(i32, i32)> {
    let rest = name.strip_prefix("c.")?.strip_suffix(".dat")?;
    let (a, b) = rest.split_once('.')?;
    fn base36(s: &str) -> Option<i32> {
        let (neg, digits) = match s.strip_prefix('-') {
            Some(d) => (true, d),
            None => (false, s),
        };
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit() || b.is_ascii_lowercase()) {
            return None;
        }
        let v = i32::from_str_radix(digits, 36).ok()?;
        Some(if neg { -v } else { v })
    }
    Some((base36(a)?, base36(b)?))
}

/// `File.listFiles` in the order Windows returns them (by name).
fn list(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|r| r.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    v.sort_by_key(|p| p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default());
    v
}

/// `func_22183_a`: the chunk files under the two levels of folders, and the top folders.
fn scan(dir: &Path, chunks: &mut Vec<ChunkFile>, folders: &mut Vec<PathBuf>) {
    for top in list(dir).into_iter().filter(|p| is_chunk_folder(p)) {
        for sub in list(&top).into_iter().filter(|p| is_chunk_folder(p)) {
            for f in list(&sub) {
                let name = f.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                if let Some((x, z)) = chunk_coords(&name) {
                    chunks.push(ChunkFile { path: f, x, z });
                }
            }
        }
        folders.push(top);
    }
}

/// `func_22181_a`: copy every chunk into its region (skipping chunks the region already has).
fn convert_chunks(
    dir: &Path,
    mut chunks: Vec<ChunkFile>,
    done: &mut i32,
    total: i32,
    progress: &mut dyn FnMut(LoadStep),
) {
    // Collections.sort (stable) by region x, then region z.
    chunks.sort_by(|a, b| (a.x >> 5).cmp(&(b.x >> 5)).then((a.z >> 5).cmp(&(b.z >> 5))));
    let mut regions: HashMap<(i32, i32), RegionFile> = HashMap::new();
    for c in chunks {
        let key = (c.x >> 5, c.z >> 5);
        if !regions.contains_key(&key) {
            // RegionFileCache creates the folder with the first region file.
            let _ = std::fs::create_dir_all(dir.join("region"));
            let path = dir.join("region").join(format!("r.{}.{}.mcr", key.0, key.1));
            if let Ok(r) = RegionFile::open(&path) {
                regions.insert(key, r);
            }
        }
        if let Some(r) = regions.get_mut(&key) {
            if !r.has_chunk(c.x & 31, c.z & 31) {
                let mut raw = Vec::new();
                let read = std::fs::File::open(&c.path)
                    .and_then(|f| flate2::read::GzDecoder::new(f).read_to_end(&mut raw));
                match read {
                    Ok(_) => {
                        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                        if z.write_all(&raw).is_ok() {
                            if let Ok(compressed) = z.finish() {
                                if let Err(e) = r.write_chunk(c.x & 31, c.z & 31, &compressed) {
                                    eprintln!("{e}");
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("{}: {e}", c.path.display()),
                }
            }
        }
        *done += 1;
        progress(LoadStep::Progress(java_round(100.0 * *done as f64 / total as f64)));
    }
}

/// `func_22182_a`: delete the old folders.
fn delete_folders(folders: Vec<PathBuf>, done: &mut i32, total: i32, progress: &mut dyn FnMut(LoadStep)) {
    for f in folders {
        let _ = std::fs::remove_dir_all(&f);
        *done += 1;
        progress(LoadStep::Progress(java_round(100.0 * *done as f64 / total as f64)));
    }
}

fn java_round(x: f64) -> i32 {
    (x + 0.5).floor() as i32
}

/// `Minecraft.convertMapFormat` + `SaveConverterMcRegion.convertMapFormat`.
pub fn convert_map_format(dir: &Path, progress: &mut dyn FnMut(LoadStep)) -> bool {
    progress(LoadStep::Title("Converting World to Scaevolus' McRegion"));
    progress(LoadStep::Text("This may take a while :)"));
    progress(LoadStep::Progress(0));
    let nether = dir.join("DIM-1");
    let (mut chunks, mut folders, mut nether_chunks, mut nether_folders) = (vec![], vec![], vec![], vec![]);
    println!("Scanning folders...");
    scan(dir, &mut chunks, &mut folders);
    if nether.exists() {
        scan(&nether, &mut nether_chunks, &mut nether_folders);
    }
    let total = (chunks.len() + nether_chunks.len() + folders.len() + nether_folders.len()) as i32;
    println!("Total conversion count is {total}");
    let mut done = 0;
    convert_chunks(dir, chunks, &mut done, total, progress);
    convert_chunks(&nether, nether_chunks, &mut done, total, progress);
    let Some(mut info) = Storage::read_level(dir) else { return false };
    info.version = 19132;
    match Storage::open(dir) {
        Ok(s) => {
            if let Err(e) = s.save_level(&info) {
                eprintln!("{e}");
            }
        }
        Err(e) => eprintln!("{e}"),
    }
    delete_folders(folders, &mut done, total, progress);
    if nether.exists() {
        delete_folders(nether_folders, &mut done, total, progress);
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn chunk_names() {
        assert_eq!(super::chunk_coords("c.0.0.dat"), Some((0, 0)));
        assert_eq!(super::chunk_coords("c.-1.1a.dat"), Some((-1, 46)));
        assert_eq!(super::chunk_coords("c.a.b.dat.bak"), None);
    }
}
