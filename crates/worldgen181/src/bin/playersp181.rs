fn main() {
    let a: Vec<String> = std::env::args().collect();
    let actual = worldgen::entity::player::verify181::sp_boundaries();
    std::fs::write(&a[1], &actual).unwrap();
    if let Some(path) = a.get(2) {
        let expected = std::fs::read(path).unwrap();
        if actual != expected {
            let at = actual
                .iter()
                .zip(&expected)
                .position(|(a, b)| a != b)
                .unwrap_or(actual.len().min(expected.len()));
            eprintln!(
                "playersp181: DIFFERS byte {}, lengths {}/{}",
                at,
                actual.len(),
                expected.len()
            );
            std::process::exit(1);
        }
        println!(
            "playersp181: EXACT (10000 action/FOV boundaries, 650 ticks, {} bytes)",
            actual.len()
        );
    }
}
