//! Strict original StatFileWriter API observations.
use super::stats_file::Syncher;
use std::collections::HashMap;
fn text(data: &mut &[u8]) -> String {
    let n = i32::from_be_bytes(data[..4].try_into().unwrap()) as usize;
    *data = &data[4..];
    let out = String::from_utf8(data[..n].to_vec()).unwrap();
    *data = &data[n..];
    out
}
fn map(map: Option<HashMap<i32, i32>>) -> String {
    let Some(map) = map else { return "null".into() };
    let mut rows: Vec<_> = map.into_iter().collect();
    rows.sort_unstable();
    rows.into_iter()
        .map(|(id, n)| format!("{id}={n};"))
        .collect()
}
pub fn check(path: &str) -> bool {
    let bytes = std::fs::read(path).unwrap();
    let mut data = bytes.as_slice();
    let count = i32::from_be_bytes(data[..4].try_into().unwrap());
    data = &data[4..];
    let guids = crate::assets::Jar::open()
        .unwrap()
        .text("achievement/map.txt")
        .unwrap();
    let parser = Syncher::fixture(&guids);
    let mut failures = 0;
    for case in 0..count {
        let input = text(&mut data);
        let expected = text(&mut data);
        let actual = match parser.parse_checked(&input) {
            Ok(m) => map(m),
            Err(e) => format!("throw:{e}"),
        };
        if actual != expected {
            eprintln!(
                "STATFILE10 case {case}: expected {expected:?}, actual {actual:?}; input {input:?}"
            );
            failures += 1;
        }
    }
    let formats = i32::from_be_bytes(data[..4].try_into().unwrap());
    data = &data[4..];
    for case in 0..formats {
        let user = text(&mut data);
        let session = text(&mut data);
        let n = i32::from_be_bytes(data[..4].try_into().unwrap());
        data = &data[4..];
        let mut entries = Vec::new();
        for _ in 0..n {
            let id = i32::from_be_bytes(data[..4].try_into().unwrap());
            let present = data[4] != 0;
            data = &data[5..];
            let value = if present {
                let v = i32::from_be_bytes(data[..4].try_into().unwrap());
                data = &data[4..];
                Some(v)
            } else {
                None
            };
            entries.push((id, value));
        }
        let expected = text(&mut data);
        let actual = parser.format_entries(
            user.strip_prefix("value:"),
            session.strip_prefix("value:"),
            entries,
        );
        if actual != expected {
            eprintln!("STATFILE10 serialization {case} differs");
            failures += 1;
        }
    }
    assert!(data.is_empty());
    println!(
        "STATFILE10: {count} original schema/grammar/checksum/hash-order cases, {formats} byte-exact serializations, {failures} differences"
    );
    failures == 0
}
