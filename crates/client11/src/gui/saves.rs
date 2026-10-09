//! `SaveConverterMcRegion` / `SaveFormatOld` as the menus use them: list worlds, look up a
//! folder's `WorldInfo`, rename, delete.

use std::path::{Path, PathBuf};
use worldgen::nbt::{self, Compound, Tag};

/// `SaveFormatComparator`
#[derive(Clone, Debug)]
pub struct SaveEntry {
    pub file_name: String,
    pub display_name: String,
    pub file_name_utf16: Option<Vec<u16>>,
    pub display_name_utf16: Option<Vec<u16>>,
    pub last_played: i64,
    pub size_on_disk: i64,
    pub needs_conversion: bool,
    pub game_mode: i32,
    pub hardcore: bool,
}

fn read_data(path: &Path) -> Option<(Compound, Compound)> {
    let bytes = std::fs::read(path).ok()?;
    let root = nbt::read_gzip(&bytes).ok()?;
    let data = root.compound("Data")?.clone();
    Some((root, data))
}

/// `func_22173_b`: the folder's level data (level.dat, else level.dat_old), if any.
pub fn world_info(saves: &Path, folder: &str) -> Option<Compound> {
    world_info_path(&saves.join(folder))
}

pub fn world_info_utf16(saves: &Path, folder: &[u16]) -> Option<Compound> {
    world_info_path(&saves.join(crate::worker::java_path_units(folder)))
}

fn world_info_path(dir: &Path) -> Option<Compound> {
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
        #[cfg(windows)]
        let file_name_utf16 = {
            use std::os::windows::ffi::OsStrExt;
            let units: Vec<u16> = d.file_name().unwrap().encode_wide().collect();
            String::from_utf16(&units).is_err().then_some(units)
        };
        #[cfg(not(windows))]
        let file_name_utf16: Option<Vec<u16>> = None;
        if let Some(info) = world_info_path(&d) {
            let mut display = info.string("LevelName");
            let units = info.string_utf16("LevelName");
            let mut display_name_utf16 = String::from_utf16(&units).is_err().then_some(units);
            if display.is_empty() {
                display = name.clone();
                display_name_utf16 = file_name_utf16.clone();
            }
            out.push(SaveEntry {
                file_name: name,
                display_name: display,
                file_name_utf16,
                display_name_utf16,
                last_played: info.long("LastPlayed"),
                size_on_disk: 0,
                needs_conversion: info.int("version") != 19132,
                game_mode: info.int("GameType"),
                hardcore: info.byte("hardcore") != 0,
            });
        }
    }
    out.sort_by(|a, b| b.last_played.cmp(&a.last_played).then_with(|| {
        let a_units = a.file_name_utf16.clone().unwrap_or_else(|| a.file_name.encode_utf16().collect());
        let b_units = b.file_name_utf16.clone().unwrap_or_else(|| b.file_name.encode_utf16().collect());
        a_units.cmp(&b_units)
    }));
    out
}

/// `String.compareTo` (UTF-16 code unit order).
pub fn java_compare(a: &str, b: &str) -> std::cmp::Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// `func_22170_a`: set LevelName in level.dat.
pub fn rename(saves: &Path, folder: &str, name: &str) {
    rename_utf16(saves, &folder.encode_utf16().collect::<Vec<_>>(), &name.encode_utf16().collect::<Vec<_>>());
}

pub fn rename_utf16(saves: &Path, folder: &[u16], name: &[u16]) {
    let p = saves.join(crate::worker::java_path_units(folder)).join("level.dat");
    if let Some((mut root, mut data)) = read_data(&p) {
        data.put("LevelName", match String::from_utf16(name) {
            Ok(name) => Tag::String(name), Err(_) => Tag::StringUtf16(name.to_vec()),
        });
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
    delete_utf16(saves, &folder.encode_utf16().collect::<Vec<_>>());
}

pub fn delete_utf16(saves: &Path, folder: &[u16]) {
    let dir = saves.join(crate::worker::java_path_units(folder));
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

#[cfg(all(test, windows))]
mod unicode_world_lifecycle11_tests {
    use super::*;
    #[test]
    fn unicode_world_first_save_reopen_list_rename_and_delete() {
        use worldgen::{World, entity::player::food::WorldSettings, storage::WorldType};
        let root=std::env::current_dir().unwrap().join("target/r11/unicode-world-lifecycle");
        std::fs::create_dir_all(&root).unwrap();
        for surrogate in [0xd83du16,0xde00] {
            let folder=vec![b'U' as u16, surrogate];
            let name=vec![b'N' as u16, surrogate];
            let dir=root.join(crate::worker::java_path_units(&folder));
            if dir.exists() { delete_utf16(&root,&folder); }
            let settings=WorldSettings::new(4181,0,false).with_world_type(WorldType::Flat);
            let (world,is_new)=World::open_with_utf16_settings(Some(&dir),4181,42,false,
                Some(&String::from_utf16_lossy(&name)),Some(&name),Some(settings),&mut |_|{}).unwrap();
            assert!(is_new);assert_eq!(world.info.name_utf16.as_deref(),Some(name.as_slice()));
            // open_with_utf16_settings has already written the first save before it returns.
            let stored=worldgen::storage::Storage::open(&dir).unwrap().load_level().unwrap();
            assert_eq!(stored.name_utf16.as_deref(),Some(name.as_slice()));drop(world);
            let entries=list(&root);let entry=entries.iter().find(|entry|entry.file_name_utf16.as_deref()==Some(folder.as_slice())).unwrap();
            assert_eq!(entry.display_name_utf16.as_deref(),Some(name.as_slice()));
            let (reopened,is_new)=World::open_with_utf16_settings(Some(&dir),-1,42,false,Some("ignored"),Some(&[b'X' as u16]),None,&mut |_|{}).unwrap();
            assert!(!is_new);assert_eq!(reopened.info.name_utf16.as_deref(),Some(name.as_slice()));drop(reopened);
            let renamed=vec![b'R' as u16, surrogate];rename_utf16(&root,&folder,&renamed);
            assert_eq!(world_info_utf16(&root,&folder).unwrap().string_utf16("LevelName"),renamed);
            assert_eq!(list(&root).iter().find(|entry|entry.file_name_utf16.as_deref()==Some(folder.as_slice())).unwrap().display_name_utf16.as_deref(),Some(renamed.as_slice()));
            if dir.exists() { delete_utf16(&root,&folder); }assert!(!dir.exists());
        }
        println!("UNICODE WORLD LIFECYCLE EXACT: two unmatched surrogate directions, native paths, initial save, reopen, list, rename and delete");
    }
}

