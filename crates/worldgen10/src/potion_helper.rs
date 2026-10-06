//! Release 1.0 PotionHelper. The expression language deliberately retains the
//! original substring bounds, flag resets, Java shifts and integer overflow.
use crate::entity::potion::PotionEffect;
use std::collections::HashMap;

pub const SUGAR: &str = "-0+1-2-3&4-4+13";
pub const GHAST_TEAR: &str = "+0-1-2-3&4-4+13";
pub const SPIDER_EYE: &str = "-0-1+2-3&4-4+13";
pub const FERMENTED_SPIDER_EYE: &str = "-0+3-4+13";
pub const GLISTERING_MELON: &str = "+0-1+2-3&4-4+13";
pub const BLAZE_POWDER: &str = "+0-1-2+3&4-4+13";
pub const MAGMA_CREAM: &str = "+0+1-2-3&4-4+13";
pub const REDSTONE: &str = "-5+6-7";
pub const GLOWSTONE: &str = "+5-6-7";
pub const GUNPOWDER: &str = "+14&13-13";
pub const NETHER_WART: &str = "+4";

pub fn ingredient(id: i32) -> Option<&'static str> {
    match id {
        353 => Some(SUGAR),
        370 => Some(GHAST_TEAR),
        375 => Some(SPIDER_EYE),
        376 => Some(FERMENTED_SPIDER_EYE),
        382 => Some(GLISTERING_MELON),
        377 => Some(BLAZE_POWDER),
        378 => Some(MAGMA_CREAM),
        331 => Some(REDSTONE),
        348 => Some(GLOWSTONE),
        289 => Some(GUNPOWDER),
        372 => Some(NETHER_WART),
        _ => None,
    }
}
pub fn bit(data: i32, index: i32) -> bool {
    data & 1i32.wrapping_shl(index as u32) != 0
}
pub fn pack_bits(data: i32, indices: [i32; 5]) -> i32 {
    indices
        .into_iter()
        .enumerate()
        .fold(0, |n, (i, b)| n | if bit(data, b) { 16 >> i } else { 0 })
}
pub fn prefix_index(data: i32) -> usize {
    pack_bits(data, [5, 4, 3, 2, 1]) as usize
}
pub fn prefix(data: i32) -> String {
    format!(
        "potion.prefix.{}",
        [
            "mundane",
            "uninteresting",
            "bland",
            "clear",
            "milky",
            "diffuse",
            "artless",
            "thin",
            "awkward",
            "flat",
            "bulky",
            "bungling",
            "buttered",
            "smooth",
            "suave",
            "debonair",
            "thick",
            "elegant",
            "fancy",
            "charming",
            "dashing",
            "refined",
            "cordial",
            "sparkling",
            "potent",
            "foul",
            "odorless",
            "rank",
            "harsh",
            "acrid",
            "gross",
            "stinky"
        ][prefix_index(data)]
    )
}
fn term(
    absent: bool,
    multiply: bool,
    negative: bool,
    comparison: i32,
    index: i32,
    factor: i32,
    data: i32,
) -> i32 {
    // func_40348_c counts no bits at all for a negative signed int.
    let count = if data > 0 {
        data.count_ones() as i32
    } else {
        0
    };
    let value = if absent {
        !bit(data, index)
    } else if comparison != -1 {
        match comparison {
            0 => count == index,
            1 => count > index,
            2 => count < index,
            _ => false,
        }
    } else {
        bit(data, index)
    } as i32;
    let value = if multiply {
        value.wrapping_mul(factor)
    } else {
        value
    };
    if negative {
        value.wrapping_neg()
    } else {
        value
    }
}
pub fn evaluate(expression: &str, data: i32) -> i32 {
    let chars: Vec<u16> = expression.encode_utf16().collect();
    evaluate_range(&chars, 0, chars.len() as i32, data)
}
fn evaluate_range(s: &[u16], start: i32, end: i32, data: i32) -> i32 {
    if start >= s.len() as i32 || end < 0 || start >= end {
        return 0;
    }
    for op in [b'|', b'&'] {
        if let Some(at) = (start as usize..s.len())
            .find(|&i| s[i] == op as u16)
            .filter(|&i| (i as i32) < end)
        {
            let left = evaluate_range(s, start, at as i32 - 1, data);
            if op == b'|' && left > 0 {
                return left;
            }
            if op == b'&' && left <= 0 {
                return 0;
            }
            let right = evaluate_range(s, at as i32 + 1, end, data);
            return if op == b'|' {
                right.max(0)
            } else if right <= 0 {
                0
            } else {
                left.max(right)
            };
        }
    }
    let (mut star, mut multiply, mut digits, mut absent, mut negative) =
        (false, false, false, false, false);
    let (mut comparison, mut index, mut factor, mut total) = (-1, 0i32, 0i32, 0i32);
    for &c in &s[start as usize..end as usize] {
        if (48..=57).contains(&c) {
            if star {
                factor = (c - 48) as i32;
                multiply = true;
            } else {
                index = index.wrapping_mul(10).wrapping_add((c - 48) as i32);
                digits = true;
            }
        } else if c == 42 {
            star = true;
        } else if matches!(c, 33 | 45 | 61 | 60 | 62 | 43) {
            if digits {
                total = total.wrapping_add(term(
                    absent, multiply, negative, comparison, index, factor, data,
                ));
                absent = false;
                star = false;
                multiply = false;
                digits = false;
                factor = 0;
                index = 0;
                comparison = -1;
                // The '-' branch does not reset the negative flag.
                if c != 45 {
                    negative = false;
                }
            }
            match c {
                33 => absent = true,
                45 => negative = true,
                61 => comparison = 0,
                60 => comparison = 2,
                62 => comparison = 1,
                _ => {}
            }
        }
    }
    if digits {
        total = total.wrapping_add(term(
            absent, multiply, negative, comparison, index, factor, data,
        ));
    }
    total
}
fn change_bit(mut data: i32, index: i32, clear: bool, toggle: bool, require: bool) -> i32 {
    let mask = 1i32.wrapping_shl(index as u32);
    if require {
        if !bit(data, index) {
            return 0;
        }
    } else if clear {
        data &= !mask;
    } else if toggle {
        data ^= mask;
    } else {
        data |= mask;
    }
    data
}
pub fn brew(mut data: i32, expression: &str) -> i32 {
    let (mut digits, mut toggle, mut clear, mut require) = (false, false, false, false);
    let mut index = 0i32;
    for c in expression.encode_utf16() {
        if (48..=57).contains(&c) {
            index = index.wrapping_mul(10).wrapping_add((c - 48) as i32);
            digits = true;
        } else if matches!(c, 33 | 45 | 43 | 38) {
            if digits {
                data = change_bit(data, index, clear, toggle, require);
                require = false;
                digits = false;
                index = 0;
                match c {
                    33 => clear = false,
                    45 => toggle = false,
                    _ => {
                        clear = false;
                        toggle = false;
                    }
                }
            }
            match c {
                33 => toggle = true,
                45 => clear = true,
                38 => require = true,
                _ => {}
            }
        }
    }
    if digits {
        data = change_bit(data, index, clear, toggle, require);
    }
    data & 32767
}
pub fn effects(data: i32, _include_unusable: bool) -> Option<Vec<PotionEffect>> {
    let mut result = Vec::new();
    // Potion.potionTypes is traversed in ascending ID, not HashMap order.
    for (id, requirement, amplified, instant, multiplier) in [
        (1, "!0 & 1 & !2 & !3 & 1+6", true, false, 1.0),
        (2, "!0 & 1 & !2 & 3 & 3+6", false, false, 0.5),
        (5, "0 & !1 & !2 & 3 & 3+6", true, false, 1.0),
        (6, "0 & !1 & 2 & !3", true, true, 1.0),
        (7, "!0 & !1 & 2 & 3", true, true, 0.5),
        (10, "0 & !1 & !2 & !3 & 0+6", true, false, 0.25),
        (12, "0 & 1 & !2 & !3 & 0+6", false, false, 1.0),
        (18, "!0 & !1 & !2 & 3 & 3+6", false, false, 0.5),
        (19, "!0 & !1 & 2 & !3 & 2+6", true, false, 0.25),
    ] {
        let mut duration = evaluate(requirement, data);
        if duration <= 0 {
            continue;
        }
        let amplifier = if amplified {
            evaluate("5", data).max(0)
        } else {
            0
        };
        if instant {
            duration = 1;
        } else {
            duration = 1200i32.wrapping_mul(
                duration
                    .wrapping_mul(3)
                    .wrapping_add(duration.wrapping_sub(1).wrapping_mul(2)),
            );
            duration >>= amplifier & 31;
            duration = (duration as f64 * multiplier + 0.5).floor() as i64 as i32;
            if data & 16384 != 0 {
                duration = (duration as f64 * 0.75 + 1.0).floor() as i64 as i32;
            }
        }
        result.push(PotionEffect::new(id, duration, amplifier));
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
pub fn potion_color(id: i32) -> i32 {
    [
        0, 8171462, 5926017, 14270531, 4866583, 9643043, 16262179, 4393481, 7889559, 5578058,
        13458603, 10044730, 14981690, 3035801, 8356754, 2039587, 2039713, 5797459, 4738376,
        5149489,
    ][id as usize]
}
pub fn color(effects: Option<&[PotionEffect]>) -> i32 {
    let Some(effects) = effects.filter(|e| !e.is_empty()) else {
        return 3694022;
    };
    let (mut r, mut g, mut b, mut count) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for e in effects {
        let c = potion_color(e.id);
        for _ in 0..=e.amplifier {
            r += ((c >> 16) & 255) as f32 / 255.0;
            g += ((c >> 8) & 255) as f32 / 255.0;
            b += (c & 255) as f32 / 255.0;
            count += 1.0;
        }
    }
    ((r / count * 255.0) as i32).wrapping_shl(16)
        | ((g / count * 255.0) as i32).wrapping_shl(8)
        | (b / count * 255.0) as i32
}
#[derive(Default)]
pub struct PotionHelper {
    colors: HashMap<i32, i32>,
}
impl PotionHelper {
    pub fn color(&mut self, data: i32, include_unusable: bool) -> i32 {
        if include_unusable {
            return color(effects(data, true).as_deref());
        }
        *self
            .colors
            .entry(data)
            .or_insert_with(|| color(effects(data, false).as_deref()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brewing_chain() {
        let awkward = brew(0, NETHER_WART);
        assert_eq!(awkward, 16);
        let speed = brew(awkward, SUGAR);
        assert_eq!(speed, 8194);
        assert_eq!(
            effects(speed, false),
            Some(vec![PotionEffect::new(1, 3600, 0)])
        );
        assert_eq!(
            effects(brew(speed, REDSTONE), false),
            Some(vec![PotionEffect::new(1, 9600, 0)])
        );
        assert_eq!(
            effects(brew(speed, GLOWSTONE), false),
            Some(vec![PotionEffect::new(1, 1800, 1)])
        );
        assert_eq!(
            effects(brew(speed, GUNPOWDER), false),
            Some(vec![PotionEffect::new(1, 2701, 0)])
        );
        assert_eq!(brew(0, SUGAR), 8192);
        assert_eq!(effects(0, false), None);
    }
    #[test]
    fn original_parser_edges() {
        assert_eq!(evaluate("0|1", 1), 0); // no space: original drops the left digit
        assert_eq!(evaluate("0 | 1", 1), 1);
        assert_eq!(evaluate("=0", -1), 1);
        assert_eq!(evaluate("0*12", 1), 2); // multiplier keeps only its last digit
        assert_eq!(brew(1, "!0!0"), 1);
        assert_eq!(brew(-1, "-31"), 32767);
        assert_eq!(prefix(16), "potion.prefix.awkward");
    }
}
