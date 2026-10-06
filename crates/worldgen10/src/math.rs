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
        i.wrapping_sub(1)
    } else {
        i
    }
}

#[inline]
pub fn floor_float(v: f32) -> i32 {
    let i = v as i32;
    if v < i as f32 {
        i.wrapping_sub(1)
    } else {
        i
    }
}

pub fn sin_table_bits() -> impl Iterator<Item = u32> {
    sin_table().iter().map(|f| f.to_bits())
}

/// Java's `(float)Math.PI`.
pub const PI_F32: f32 = std::f64::consts::PI as f32;

/// MathHelper.func_35599_c, retaining Java's saturated cast and overflow.
pub fn floor_long(v:f64)->i64 {let i=v as i64;if v<i as f64{i.wrapping_sub(1)}else{i}}
pub fn abs_int(v:i32)->i32 {if v>=0{v}else{v.wrapping_neg()}}
pub fn next_int_inclusive(r:&mut crate::JavaRandom,min:i32,max:i32)->i32 {
    if min>=max{min}else{r.next_int(max.wrapping_sub(min).wrapping_add(1)).wrapping_add(min)}
}
pub fn sqrt_float(v:f32)->f32 {(v as f64).sqrt() as f32}
pub fn sqrt_double(v:f64)->f32 {v.sqrt() as f32}
pub fn abs_float(v:f32)->f32 {if v>=0.0 {v}else{-v}}
pub fn abs_max(mut a:f64,mut b:f64)->f64 {
    if a<0.0 {a=-a;}if b<0.0 {b=-b;}if a>b {a}else{b}
}
pub fn bucket_int(a:i32,b:i32)->i32 {
    if a<0 {a.wrapping_neg().wrapping_sub(1).wrapping_div(b).wrapping_neg().wrapping_sub(1)}else{a.wrapping_div(b)}
}
pub fn string_null_or_empty(s:Option<&str>)->bool {s.is_none_or(str::is_empty)}
