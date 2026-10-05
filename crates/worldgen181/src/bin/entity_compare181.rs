use std::{env, fs, process};
fn main() {
    let a: Vec<_> = env::args().collect();
    if a.len() != 4 {
        eprintln!("usage: entity_compare181 CASE TICKS ORACLE_FILE");
        process::exit(2);
    }
    let actual = match a[1].as_str() {
        "watchers" | "coordinates" | "light" | "registry" => worldgen::entity::api_verify181::replay(&a[1]),
        "creeperdeath4" | "sun4" | "network4" | "lightning4" | "retaliation4" | "impact4" | "flying4" | "explosionblocks4" | "creeper4" => worldgen::entity::round4_verify181::replay(&a[1]),
        "combat" => worldgen::entity::combat_verify181::replay(),
        "explosion" => worldgen::entity::combat_verify181::explosion_replay(),
        "slimebranches" | "skeletonbranches" | "wolfbranches" | "vehiclebranches" | "fireballbranches" | "spawnbranches" | "spawnorder" | "factorybranches" => worldgen::entity::branches_verify181::replay(&a[1]),
        _ => worldgen::entity::verify181::replay(&a[1], a[2].parse().unwrap()),
    };
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
