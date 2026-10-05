use std::{env, fs, process};
fn main() {
    let a: Vec<_> = env::args().collect();
    if a.len() != 4 {
        eprintln!("usage: entity_compare181 CASE TICKS ORACLE_FILE");
        process::exit(2);
    }
    let actual = worldgen::entity::verify181::replay(&a[1], a[2].parse().unwrap());
    let expected = fs::read(&a[3]).unwrap();
    if actual == expected {
        println!("EXACT: entities181 {} ({} bytes)", a[1], actual.len());
        return;
    }
    let path = std::path::Path::new(&a[3]).with_extension("rust.bin");
    fs::write(&path, &actual).unwrap();
    let offset = actual
        .iter()
        .zip(&expected)
        .position(|(a, b)| a != b)
        .unwrap_or(actual.len().min(expected.len()));
    eprintln!(
        "DIFF: {} byte {} (Rust {} bytes, Java {} bytes); actual {}",
        a[1],
        offset,
        actual.len(),
        expected.len(),
        path.display()
    );
    process::exit(1);
}
