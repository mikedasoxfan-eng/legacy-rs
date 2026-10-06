pub fn canonical(n: &worldgen::nbt::Compound) -> Vec<u8> {
    let mut bytes = Vec::new();
    worldgen::nbt::write_root_canonical(&mut bytes, n).unwrap();
    bytes
}
