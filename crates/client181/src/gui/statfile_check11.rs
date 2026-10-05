//! Compare constructor disk effects and original UTF-16/charset API outcomes.
use super::stats_file::{decode_file, md5_units, Syncher};
use std::path::Path;
struct Input<'a>(&'a [u8]);
impl Input<'_> {
    fn int(&mut self) -> usize {
        let n = u32::from_be_bytes(self.0[..4].try_into().unwrap()) as usize;
        self.0 = &self.0[4..];
        n
    }
    fn raw(&mut self) -> Vec<u8> {
        let n = self.int();
        let b = self.0[..n].to_vec();
        self.0 = &self.0[n..];
        b
    }
    fn text(&mut self) -> String {
        String::from_utf8(self.raw()).unwrap()
    }
    fn units(&mut self) -> Vec<u16> {
        let n = self.int();
        let u = self.0[..n * 2]
            .chunks_exact(2)
            .map(|b| u16::from_be_bytes(b.try_into().unwrap()))
            .collect();
        self.0 = &self.0[n * 2..];
        u
    }
    fn flag(&mut self) -> bool {
        let b = self.0[0] != 0;
        self.0 = &self.0[1..];
        b
    }
}
fn map(m: Option<std::collections::HashMap<i32, i32>>) -> String {
    let Some(m) = m else { return "null".into() };
    let mut rows: Vec<_> = m.into_iter().collect();
    rows.sort_unstable();
    rows.into_iter()
        .map(|(id, n)| format!("{id}={n};"))
        .collect()
}
fn make(d: &Path, c: usize) {
    if c == 0 {
        return;
    }
    if c == 1 {
        std::fs::write(d, [1]).unwrap();
        return;
    }
    std::fs::create_dir(d).unwrap();
    if c == 2 {
        std::fs::write(d.join("stats"), [2]).unwrap();
    }
    if c >= 3 {
        std::fs::write(d.join("stats_Player_unsent.dat"), b"{}").unwrap();
        std::fs::write(d.join("stats_other.dat"), [3]).unwrap();
        std::fs::write(d.join("stats_other.tmp"), [4]).unwrap();
        std::fs::create_dir(d.join("stats_directory.dat")).unwrap();
    }
    if c == 4 || c == 5 {
        std::fs::create_dir(d.join("stats")).unwrap();
        std::fs::write(d.join("stats/stats_other.dat"), [9]).unwrap();
    }
    if c == 5 {
        std::fs::create_dir(d.join("stats/stats_directory.dat")).unwrap();
    }
}
fn disk(d: &Path) -> String {
    fn scan(d: &Path, path: String, rows: &mut std::collections::BTreeMap<String, String>) {
        if !d.exists() {
            rows.insert(path, "missing".into());
        } else if !d.is_dir() {
            rows.insert(path, format!("{:?}", std::fs::read(d).unwrap()));
        } else {
            rows.insert(path.clone(), "dir".into());
            for f in std::fs::read_dir(d).unwrap() {
                let f = f.unwrap();
                scan(
                    &f.path(),
                    format!("{path}/{}", f.file_name().to_string_lossy()),
                    rows,
                );
            }
        }
    }
    let mut rows = Default::default();
    scan(d, "".into(), &mut rows);
    rows.into_iter().map(|(k, v)| format!("{k}={v};")).collect()
}
pub fn check(path: &str) -> bool {
    let bytes = std::fs::read(path).unwrap();
    let mut input = Input(&bytes);
    let charset = input.text();
    assert_eq!(charset, "Cp1252");
    let guids = crate::assets::Jar::open()
        .unwrap()
        .text("achievement/map.txt")
        .unwrap();
    let parser = Syncher::fixture(&guids);
    let mut failures = 0;
    let count = input.int();
    for c in 0..count {
        let s = input.units();
        let expected = input.text();
        let actual = match parser.parse_units_checked(&s) {
            Ok(m) => map(m),
            Err(e) => format!("throw:{e}"),
        };
        if actual != expected {
            if failures < 15 {
                eprintln!("STATFILE11 {c}: {s:?}: expected {expected}, actual {actual}");
            }
            failures += 1;
        }
    }
    let root = std::env::current_dir()
        .unwrap()
        .join(format!("target/statfile11-rust-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let ctors = input.int();
    for c in 0..ctors {
        let scenario = input.int();
        let session = input.flag();
        let expected = input.text();
        let expected_disk = input.text();
        let d = root.join(format!("case{c}"));
        make(&d, scenario);
        let actual = match Syncher::open_checked(&d, session.then_some("Player"), &guids) {
            Ok((s, _)) => {
                s.wait_idle();
                "ok"
            }
            Err(e) => e,
        };
        let actual_disk = disk(&d);
        if actual != expected || actual_disk != expected_disk {
            eprintln!("STATFILE11 ctor {c}: {actual} != {expected}; disk {actual_disk:?} != {expected_disk:?}");
            failures += 1;
        }
    }
    let hashes = input.int();
    for c in 0..hashes {
        let mut salt = input.units();
        salt.extend("abc".encode_utf16());
        let expected = input.text();
        let actual = md5_units(&salt).unwrap_or_else(|e| format!("throw:{e}"));
        if actual != expected {
            eprintln!("STATFILE11 MD5 {c}: {actual} != {expected}");
            failures += 1;
        }
    }
    let formats = input.int();
    for c in 0..formats {
        let user = input.units();
        let session = input.units();
        let expected = input.text();
        match parser.format_units(
            Some(&user),
            Some(&session),
            [(1000, Some(c as i32)), (5242880, Some(-(c as i32)))],
        ) {
            Ok(actual) => {
                assert_eq!(expected, "ok");
                let formatted = input.units();
                let raw = input.raw();
                if actual != formatted || super::stats_file::encode_units(&actual) != raw {
                    eprintln!("STATFILE11 charset serialization {c} differs");
                    failures += 1;
                }
            }
            Err(e) => {
                if e != expected {
                    eprintln!("STATFILE11 charset serializer {c}: {e} != {expected}");
                    failures += 1;
                }
            }
        }
    }
    let decoders = input.int();
    for c in 0..decoders {
        let bytes = input.raw();
        let expected = input.units();
        if decode_file(&bytes).encode_utf16().collect::<Vec<_>>() != expected {
            eprintln!("STATFILE11 decode {c} differs");
            failures += 1;
        }
    }
    assert!(input.0.is_empty());
    println!("STATFILE11: {count} malformed/UTF16, {ctors} constructor disk/failure, {hashes} MD5 charset, {formats} UTF16/encoded serializations, {decoders} decoder cases: {failures} differences");
    failures == 0
}
