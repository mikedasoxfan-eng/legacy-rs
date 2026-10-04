//! `SaveConverterMcRegion` / `SaveFormatOld` as the menus use them: list worlds, look up a
//! folder's `WorldInfo`, rename, delete.

use std::path::{Path, PathBuf};
use worldgen::nbt::{self, Compound, Tag};

/// `SaveFormatComparator`
#[derive(Clone, Debug)]
pub struct SaveEntry {
    pub file_name: String,
    pub display_name: String,
    pub last_played: i64,
    pub size_on_disk: i64,
    pub needs_conversion: bool,
}

fn read_data(path: &Path) -> Option<(Compound, Compound)> {
    let bytes = std::fs::read(path).ok()?;
    let root = nbt::read_gzip(&bytes).ok()?;
    let data = root.compound("Data")?.clone();
    Some((root, data))
}

/// `func_22173_b`: the folder's level data (level.dat, else level.dat_old), if any.
pub fn world_info(saves: &Path, folder: &str) -> Option<Compound> {
    let dir = saves.join(folder);
    if !dir.exists() {
        return None;
    }
    for name in ["level.dat", "level.dat_old"] {
        let p = dir.join(name);
        if p.exists() {
            if let Some((_, d)) = read_data(&p) {
                return Some(d);
            }
        }
    }
    None
}

/// `func_22176_b` then `Collections.sort`: newest first, then by folder name.
pub fn list(saves: &Path) -> Vec<SaveEntry> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(saves) else { return out };
    let mut dirs: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).collect();
    dirs.sort();
    for d in dirs {
        let name = d.file_name().unwrap().to_string_lossy().into_owned();
        if let Some(info) = world_info(saves, &name) {
            let mut display = info.string("LevelName");
            if display.is_empty() {
                display = name.clone();
            }
            out.push(SaveEntry {
                file_name: name,
                display_name: display,
                last_played: info.long("LastPlayed"),
                size_on_disk: info.long("SizeOnDisk"),
                needs_conversion: info.int("version") != 19132,
            });
        }
    }
    out.sort_by(|a, b| b.last_played.cmp(&a.last_played).then_with(|| java_compare(&a.file_name, &b.file_name)));
    out
}

/// `String.compareTo` (UTF-16 code unit order).
pub fn java_compare(a: &str, b: &str) -> std::cmp::Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// `func_22170_a`: set LevelName in level.dat.
pub fn rename(saves: &Path, folder: &str, name: &str) {
    let p = saves.join(folder).join("level.dat");
    if let Some((mut root, mut data)) = read_data(&p) {
        data.put("LevelName", Tag::String(name.to_string()));
        root.put("Data", Tag::Compound(data));
        match nbt::write_gzip(&root) {
            Ok(b) => {
                if let Err(e) = std::fs::write(&p, b) {
                    eprintln!("rename failed: {e}");
                }
            }
            Err(e) => eprintln!("rename failed: {e}"),
        }
    }
}

/// `func_22172_c`: delete the world folder.
pub fn delete(saves: &Path, folder: &str) {
    let dir = saves.join(folder);
    if dir.starts_with(saves) && !folder.is_empty() && dir != saves {
        if let Err(e) = std::fs::remove_dir_all(&dir) {
            eprintln!("could not delete {}: {e}", dir.display());
        }
    }
}

/// `GuiCreateWorld.generateUnusedFolderName`
pub fn unused_folder_name(saves: &Path, mut name: String) -> String {
    while world_info(saves, &name).is_some() {
        name.push('-');
    }
    name
}

/// `new SimpleDateFormat().format(date)` for the en_US default locale: `M/d/yy h:mm a`,
/// in the local time zone.
pub fn format_date(millis: i64) -> String {
    use chrono::{Local, TimeZone, Timelike, Datelike};
    let Some(t) = Local.timestamp_millis_opt(millis).single() else { return String::new() };
    let (pm, h12) = t.hour12();
    format!("{}/{}/{:02} {}:{:02} {}", t.month(), t.day(), t.year().rem_euclid(100), h12, t.minute(), if pm { "PM" } else { "AM" })
}
