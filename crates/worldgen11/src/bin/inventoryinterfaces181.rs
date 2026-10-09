fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = worldgen::container::verify_interfaces::replay();
    std::fs::write(&args[1], &out).unwrap();
    let expected = std::fs::read(&args[2]).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "INVENTORY INTERFACES DIFFER at byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        std::process::exit(1);
    }
    println!(
        "INVENTORY INTERFACES EXACT: 2400 original IInventory/Slot operations ({} bytes)",
        out.len()
    );
}
