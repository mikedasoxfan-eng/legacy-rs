//! TexturePackList / TexturePackCustom. Cache identity is filename:length:lastModified.
use crate::assets::Jar;
use crate::render::gpu::Image;
use std::path::{Path, PathBuf};

pub struct Pack {
    pub name: String,
    pub first: Option<String>,
    pub second: Option<String>,
    pub thumbnail: Option<Image>,
    pub path: Option<PathBuf>,
    pub key: String,
    opened: bool,
}

pub struct TexturePackList {
    pub folder: PathBuf,
    pub available: Vec<Pack>,
    pub selected: usize,
    current: String,
}

impl TexturePackList {
    pub fn new(dir: &Path, skin: &str) -> Self {
        let folder = dir.join("texturepacks");
        let _ = std::fs::create_dir_all(&folder);
        let mut list = Self {
            folder,
            available: vec![Pack {
                name: "Default".into(),
                first: Some("The default look of Minecraft".into()),
                second: None,
                thumbnail: Jar::open().and_then(|mut j| j.image("pack.png")).ok(),
                path: None,
                key: String::new(),
                opened: true,
            }],
            selected: 0,
            current: skin.into(),
        };
        list.update();
        list.available[list.selected].opened = true;
        list
    }

    pub fn update(&mut self) {
        let mut old = std::mem::take(&mut self.available);
        let default = old.remove(0);
        let mut available = vec![default];
        if let Ok(files) = std::fs::read_dir(&self.folder) {
            // File.listFiles preserves the filesystem's enumeration order.
            for entry in files.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().into_owned();
                if !path.is_file() || !name.to_lowercase().ends_with(".zip") {
                    continue;
                }
                let Ok(meta) = entry.metadata() else { continue };
                let millis = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map_or(0, |d| d.as_millis());
                let key = format!("{name}:{}:{millis}", meta.len());
                if let Some(i) = old.iter().position(|p| p.key == key) {
                    available.push(old.remove(i));
                    continue;
                }
                let mut pack = Pack {
                    name,
                    first: None,
                    second: None,
                    thumbnail: None,
                    path: Some(path.clone()),
                    key,
                    opened: false,
                };
                // Vanilla keeps broken ZIPs in the list: func_6485_a catches its own exceptions.
                if let Ok(mut zip) = Jar::archive(&path) {
                    if let Ok(bytes) = zip.bytes("pack.txt") {
                        // Java 8 on Windows uses windows-1252 for InputStreamReader's default.
                        let text = windows_1252(&bytes).replace("\r\n", "\n").replace('\r', "\n");
                        let mut lines = text.split_terminator('\n');
                        let mut next = || {
                            lines
                                .next()
                                .map(|line| String::from_utf16_lossy(&line.encode_utf16().take(34).collect::<Vec<_>>()))
                        };
                        pack.first = next();
                        pack.second = next();
                    }
                    pack.thumbnail = zip.image("pack.png").ok();
                }
                available.push(pack);
            }
        }
        self.selected = available.iter().position(|p| p.name == self.current).unwrap_or(0);
        self.available = available;
    }

    pub fn select(&mut self, index: usize) -> bool {
        if self.selected == index {
            return false;
        }
        self.available[self.selected].opened = false;
        self.selected = index;
        self.available[index].opened = true;
        self.current.clone_from(&self.available[index].name);
        true
    }

    pub fn path(&self) -> Option<&Path> {
        // updateAvaliableTexturePacks chooses new cache objects without opening their ZIP.
        // A replaced selected pack therefore reads the jar until switched away and back.
        let pack = &self.available[self.selected];
        if pack.opened {
            pack.path.as_deref()
        } else {
            None
        }
    }
}

/// RenderEngine.setupTexture / TextureFX: integer division, before alpha is copied.
pub fn anaglyph(rgba: &mut [u8]) {
    for c in rgba.chunks_exact_mut(4) {
        let [r, g, b] = [c[0] as u32, c[1] as u32, c[2] as u32];
        c[0] = ((r * 30 + g * 59 + b * 11) / 100) as u8;
        c[1] = ((r * 30 + g * 70) / 100) as u8;
        c[2] = ((r * 30 + b * 70) / 100) as u8;
    }
}

pub(crate) fn windows_1252(bytes: &[u8]) -> String {
    const HIGH: [char; 32] = [
        '\u{20ac}', '\u{fffd}', '\u{201a}', '\u{192}', '\u{201e}', '\u{2026}', '\u{2020}', '\u{2021}', '\u{2c6}',
        '\u{2030}', '\u{160}', '\u{2039}', '\u{152}', '\u{fffd}', '\u{17d}', '\u{fffd}', '\u{fffd}', '\u{2018}',
        '\u{2019}', '\u{201c}', '\u{201d}', '\u{2022}', '\u{2013}', '\u{2014}', '\u{2dc}', '\u{2122}', '\u{161}',
        '\u{203a}', '\u{153}', '\u{fffd}', '\u{17e}', '\u{178}',
    ];
    bytes.iter().map(|&b| if (0x80..0xa0).contains(&b) { HIGH[(b - 0x80) as usize] } else { b as char }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn write_pack(path: &Path, description: &[u8]) {
        let mut zip = zip::ZipWriter::new(std::fs::File::create(path).unwrap());
        zip.start_file("pack.txt", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(description).unwrap();
        zip.start_file("oracle-resource.txt", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"selected archive").unwrap();
        zip.finish().unwrap();
    }
    #[test]
    fn scan_cache_selection_replacement_and_resource_fallback() {
        let dir = std::env::temp_dir().join(format!(
            "beta-packs-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(dir.join("texturepacks")).unwrap();
        let path = dir.join("texturepacks/Oracle.ZIP");
        write_pack(&path, b"0123456789012345678901234567890123456789\r\n\x80 second line\rthird ignored");
        std::fs::write(dir.join("texturepacks/Broken.zip"), b"not a zip").unwrap();
        std::fs::create_dir(dir.join("texturepacks/Directory.zip")).unwrap();
        let mut list = TexturePackList::new(&dir, "missing.zip");
        assert_eq!(list.selected, 0);
        assert_eq!(list.current, "missing.zip");
        assert_eq!(list.available.len(), 3);
        let index = list.available.iter().position(|p| p.name == "Oracle.ZIP").unwrap();
        assert_eq!(list.available[index].first.as_deref(), Some("0123456789012345678901234567890123"));
        assert_eq!(list.available[index].second.as_deref(), Some("\u{20ac} second line"));
        assert!(list.select(index));
        let mut resources = Jar::with_pack(list.path()).unwrap();
        assert_eq!(resources.bytes("/oracle-resource.txt").unwrap(), b"selected archive");
        assert_eq!(
            resources.bytes("/font/default.png").unwrap(),
            Jar::open().unwrap().bytes("font/default.png").unwrap()
        );
        drop(resources);
        list.available[index].first = Some("cache object retained".into());
        list.update();
        assert_eq!(list.available[list.selected].first.as_deref(), Some("cache object retained"));
        assert!(list.path().is_some());
        write_pack(&path, b"replacement with a different file length\nnew second");
        list.update();
        assert_eq!(list.available[list.selected].first.as_deref(), Some("replacement with a different file "));
        assert!(list.path().is_none(), "vanilla replacement objects are not opened by scanning");
        let index = list.selected;
        assert!(!list.select(index));
        assert!(list.select(0));
        assert!(list.select(index));
        assert!(list.path().is_some());
        std::fs::remove_file(&path).unwrap();
        list.update();
        assert_eq!(list.selected, 0);
        assert_eq!(list.current, "Oracle.ZIP");
        std::fs::remove_file(dir.join("texturepacks/Broken.zip")).unwrap();
        std::fs::remove_dir(dir.join("texturepacks/Directory.zip")).unwrap();
        std::fs::remove_dir(dir.join("texturepacks")).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }
}
