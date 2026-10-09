use std::{env, fs};
fn main() {
    let a: Vec<_> = env::args().collect();
    let actual = worldgen::entity::round2_verify10::run(&a[1]);
    fs::write(format!("{}.rust", a[2]), &actual).unwrap();
    let expected = fs::read(&a[2]).unwrap();
    if actual == expected {
        println!("EXACT: {} ({} bytes)", a[1], actual.len());
    } else {
        let at = actual
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(actual.len().min(expected.len()));
        let fields = fs::read_to_string(format!("{}.fields", a[2])).unwrap_or_default();
        let field = fields
            .lines()
            .find(|s| {
                let p: Vec<_> = s.split('\t').collect();
                let n = p[0].parse::<usize>().unwrap();
                let size = p[1].parse::<usize>().unwrap();
                at >= n && at < n + size
            })
            .unwrap_or(
                "no scalar label; use tools/r10/diagnose_replay10.py for inherited replay layouts",
            );
        eprintln!(
            "DIFFERS: {} byte {} ({}) Java length {}, Rust length {}",
            a[1],
            at,
            field,
            expected.len(),
            actual.len()
        );
        std::process::exit(1);
    }
}
