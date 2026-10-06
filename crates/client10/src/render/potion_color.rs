//! ItemPotion's presentation-only color/instant-effect query. No brewing,
//! effect duration, inventory or simulation logic is implemented here.
// PotionHelper's nine enabled effect requirements each select one low nibble.
// Amplifier bit 5 still changes the repeated float accumulation in its color query.
pub fn properties(damage: i32) -> (u32, bool) {
    let (color, amplified, instant): (u32, bool, bool) = match damage & 15 {
        2 => (8171462, true, false),
        10 => (5926017, false, false),
        9 => (9643043, true, false),
        5 => (16262179, true, true),
        12 => (4393481, true, true),
        1 => (13458603, true, false),
        3 => (14981690, false, false),
        8 => (4738376, false, false),
        4 => (5149489, true, false),
        _ => return (3694022, false),
    };
    let amplifier = if amplified && damage & (1 << 5) != 0 {
        1
    } else {
        0
    };
    let mut rgb = [0.0f32; 3];
    let mut count = 0.0f32;
    for _ in 0..=amplifier {
        for (i, value) in rgb.iter_mut().enumerate() {
            *value += ((color >> (16 - i * 8)) & 255) as f32 / 255.0f32;
        }
        count += 1.0;
    }
    let channels = rgb.map(|v| (v / count * 255.0f32) as u32);
    (
        (channels[0] << 16) | (channels[1] << 8) | channels[2],
        instant,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn matches_the_original_source_palette_checksum() {
        // FNV-1a over big-endian int color + boolean from unchanged 1.0
        // PotionHelper, for metadata 0..65536. Reproduce with target/check_palette.py.
        let mut hash = 14695981039346656037u64;
        for damage in 0..65536 {
            let (color, instant) = super::properties(damage);
            for byte in color.to_be_bytes().into_iter().chain([instant as u8]) {
                hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
            }
            assert_eq!(super::properties(damage | i32::MIN), (color, instant));
        }
        assert_eq!(hash, 0xe80fab69e5546325);
    }
}
