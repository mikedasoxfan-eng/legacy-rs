//! SaveFormatOld: five legacy slots, fallback reads, rename and loader creation.
use crate::{
    nbt::{self, Tag},
    storage::{LevelInfo, Storage},
    storage181::{self, SaveFormatComparator181},
};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

pub struct SaveFormatOld181 {
    pub directory: PathBuf,
}
impl SaveFormatOld181 {
    pub fn new(directory: &Path) -> Self {
        if !directory.exists() {
            let _ = fs::create_dir_all(directory);
        }
        Self {
            directory: directory.to_path_buf(),
        }
    }
    pub fn name(&self) -> &'static str {
        "Old Format"
    }
    pub fn world_info(&self, name: &str) -> Option<LevelInfo> {
        Storage::read_level(&self.directory.join(name))
    }
    pub fn worlds(&self) -> Vec<SaveFormatComparator181> {
        (1..=5)
            .filter_map(|slot| {
                let name = format!("World{slot}");
                let info = self.world_info(&name)?;
                Some(SaveFormatComparator181 {
                    file_name: name,
                    display_name: String::new(),
                    last_played: info.last_played,
                    size_on_disk: info.size_on_disk,
                    game_type: info.game_type,
                    requires_conversion: false,
                })
            })
            .collect()
    }
    /// The old implementation does not flush RegionFileCache.
    pub fn flush_cache(&self) {}
    pub fn rename(&self, name: &str, new_name: &str) {
        let path = self.directory.join(name).join("level.dat");
        if !path.exists() {
            return;
        }
        let Ok(mut root) = storage181::read_gzip_file(&path) else {
            return;
        };
        // Missing Data returns a detached empty compound in NBTTagCompound;
        // a wrong type throws before opening/truncating the output file.
        match root.get("Data") {
            Some(Tag::Compound(data)) => {
                let mut data = data.clone();
                data.put("LevelName", Tag::String(new_name.into()));
                root.put("Data", Tag::Compound(data));
            }
            None => (),
            _ => return,
        }
        if let Ok(mut file) = fs::File::create(path) {
            if let Ok(bytes) = nbt::write_gzip(&root) {
                let _ = file.write_all(&bytes);
            }
        }
    }
    pub fn delete(&self, name: &str) {
        fn remove(path: &Path) {
            if path.is_dir() {
                if let Ok(children) = fs::read_dir(path) {
                    for child in children.flatten() {
                        remove(&child.path());
                    }
                }
                let _ = fs::remove_dir(path);
            } else {
                let _ = fs::remove_file(path);
            }
        }
        let path = self.directory.join(name);
        if path.exists() {
            remove(&path);
        }
    }
    pub fn loader(&self, name: &str, players: bool) -> io::Result<Storage> {
        Storage::open_with_players(&self.directory.join(name), players)
    }
    pub fn is_old_map_format(&self, _name: &str) -> bool {
        false
    }
    pub fn convert_map_format(&self, _name: &str) -> bool {
        false
    }
}
