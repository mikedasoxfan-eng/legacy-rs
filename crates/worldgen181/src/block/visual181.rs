//! Immutable block texture/render defaults and the original creative item list.
//! Regenerate visual181.bin with oracle181 mode wbvisual (original bytecode).
use crate::entity::player::ItemStack;
const DATA: &[u8] = include_bytes!("visual181.bin");
const SIZE: usize = 4 + 6 * 4 + 16 * 7 * 4;
fn int(at: usize) -> i32 {
    i32::from_le_bytes(DATA[at..at + 4].try_into().unwrap())
}
fn base(id: u8, fancy: bool) -> usize {
    (fancy as usize * 256 + id as usize) * SIZE
}
pub fn render_type(id: u8) -> i32 {
    int(base(id, false))
}
pub fn texture_from_side(id: u8, side: usize, fancy: bool) -> i32 {
    assert!(side < 6);
    int(base(id, fancy) + 4 + side * 4)
}
pub fn texture_result(id: u8, side: usize, metadata: usize, fancy: bool) -> Result<i32, ()> {
    assert!(side < 6 && metadata < 16);
    let v = int(base(id, fancy) + 28 + metadata * 28 + 4 + side * 4);
    if v == i32::MIN {
        Err(())
    } else {
        Ok(v)
    }
}
pub fn texture(id: u8, side: usize, metadata: usize, fancy: bool) -> i32 {
    texture_result(id, side, metadata, fancy)
        .expect("original block texture metadata throws ArrayIndexOutOfBoundsException")
}
pub fn render_color(id: u8, metadata: usize, grass: &[u32]) -> i32 {
    assert!(metadata < 16);
    if id == 2 {
        lookup(grass, 0.5, 1.0) as i32
    } else {
        int(base(id, false) + 28 + metadata * 28)
    }
}
pub fn creative_stacks() -> Vec<ItemStack> {
    let at = SIZE * 512;
    let count = int(at) as usize;
    (0..count)
        .map(|k| {
            let p = at + 4 + k * 12;
            ItemStack::new(int(p), int(p + 4), int(p + 8))
        })
        .collect()
}
pub fn lookup(colors: &[u32], temperature: f64, mut rainfall: f64) -> u32 {
    rainfall *= temperature;
    let x = ((1.0 - temperature) * 255.0) as i32;
    let y = ((1.0 - rainfall) * 255.0) as i32;
    colors[(y << 8 | x) as usize]
}
/// The terrain integration supplies the original WorldChunkManager float climate.
pub fn color_multiplier(
    id: u8,
    metadata: i32,
    x: i32,
    y: i32,
    z: i32,
    grass: &[u32],
    foliage: &[u32],
    mut climate: impl FnMut(i32, i32) -> (f32, f32),
) -> i32 {
    match id {
        55 => 8388608,
        104 | 105 => {
            let r = metadata.wrapping_mul(32);
            let g = 255i32.wrapping_sub(metadata.wrapping_mul(8));
            let b = metadata.wrapping_mul(4);
            r.wrapping_shl(16) | g.wrapping_shl(8) | b
        }
        18 if metadata & 1 != 0 => 6396257,
        18 if metadata & 2 != 0 => 8431445,
        31 if metadata == 0 => 16777215,
        2 | 18 | 31 | 106 => {
            let (mut x, mut z) = (x, z);
            if id == 31 {
                let mut n = x
                    .wrapping_mul(3129871)
                    .wrapping_add(z.wrapping_mul(6129781))
                    .wrapping_add(y) as i64;
                n = n
                    .wrapping_mul(n)
                    .wrapping_mul(42317861)
                    .wrapping_add(n.wrapping_mul(11));
                x = x.wrapping_add(((n >> 14 & 31) - 16) as i32);
                z = z.wrapping_add(((n >> 24 & 31) - 16) as i32);
            }
            let (t, r) = climate(x, z);
            lookup(
                if matches!(id, 18 | 106) {
                    foliage
                } else {
                    grass
                },
                t as f64,
                r as f64,
            ) as i32
        }
        _ => 16777215,
    }
}
