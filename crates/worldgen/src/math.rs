//! `MathHelper`. The sine table is computed once with the same double→float
//! narrowing the Java static initializer uses.

use std::sync::OnceLock;

fn sin_table() -> &'static [f32] {
    static TABLE: OnceLock<Vec<f32>> = OnceLock::new();
    TABLE.get_or_init(|| {
        (0..65536)
            .map(|i| (i as f64 * std::f64::consts::PI * 2.0 / 65536.0).sin() as f32)
            .collect()
    })
}

#[inline]
pub fn sin(v: f32) -> f32 {
    sin_table()[((v * 10430.378f32) as i32 & 0xffff) as usize]
}

#[inline]
pub fn cos(v: f32) -> f32 {
    sin_table()[((v * 10430.378f32 + 16384.0f32) as i32 & 0xffff) as usize]
}

#[inline]
pub fn floor_double(v: f64) -> i32 {
    let i = v as i32;
    if v < i as f64 {
        i - 1
    } else {
        i
    }
}

#[inline]
pub fn floor_float(v: f32) -> i32 {
    let i = v as i32;
    if v < i as f32 {
        i - 1
    } else {
        i
    }
}

pub fn sin_table_bits() -> impl Iterator<Item = u32> {
    sin_table().iter().map(|f| f.to_bits())
}

/// Java's `(float)Math.PI`.
pub const PI_F32: f32 = std::f64::consts::PI as f32;
