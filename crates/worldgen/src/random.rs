//! `java.util.Random`, bit for bit. Every generator stage draws from one of these,
//! so call order and argument values must match the Java source exactly.

const MULTIPLIER: i64 = 0x5DEECE66D;
const ADDEND: i64 = 0xB;
const MASK: i64 = (1 << 48) - 1;

#[derive(Clone, Debug)]
pub struct JavaRandom {
    seed: i64,
    gaussian: Option<f64>,
}

impl JavaRandom {
    pub fn new(seed: i64) -> Self {
        JavaRandom {
            seed: (seed ^ MULTIPLIER) & MASK,
            gaussian: None,
        }
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & MASK;
        self.gaussian = None;
    }
    /// Non-consuming RNG fingerprint for oracle comparisons.
    pub fn state(&self) -> i64 {
        self.seed
    }

    #[inline]
    fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND)) & MASK;
        (self.seed >> (48 - bits)) as i32
    }

    /// Java throws IllegalArgumentException for `bound <= 0`; we panic.
    pub fn next_int(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "nextInt bound must be positive, got {bound}");
        if bound & bound.wrapping_neg() == bound {
            return ((bound as i64 * self.next(31) as i64) >> 31) as i32;
        }
        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits.wrapping_sub(val).wrapping_add(bound - 1) >= 0 {
                return val;
            }
        }
    }

    pub fn next_int_unbounded(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_long(&mut self) -> i64 {
        ((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
    }

    pub fn next_bool(&mut self) -> bool {
        self.next(1) != 0
    }

    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }

    pub fn next_double(&mut self) -> f64 {
        let hi = (self.next(26) as i64) << 27;
        let lo = self.next(27) as i64;
        (hi + lo) as f64 * (1.0 / (1i64 << 53) as f64)
    }
    /// java.util.Random uses StrictMath, not HotSpot's Math intrinsic, here.
    pub fn next_gaussian(&mut self) -> f64 {
        if let Some(v) = self.gaussian.take() {
            return v;
        }
        loop {
            let v1 = 2.0 * self.next_double() - 1.0;
            let v2 = 2.0 * self.next_double() - 1.0;
            let s = v1 * v1 + v2 * v2;
            if s >= 1.0 || s == 0.0 {
                continue;
            }
            let multiplier = (-2.0 * crate::jmath::fdlibm_log(s) / s).sqrt();
            self.gaussian = Some(v2 * multiplier);
            return v1 * multiplier;
        }
    }
    pub fn gaussian_cache(&self) -> Option<f64> {
        self.gaussian
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference values from `new java.util.Random(42)` on JDK 8.
    #[test]
    fn matches_java_seed_42() {
        let mut r = JavaRandom::new(42);
        assert_eq!(r.next_int(10), 0);
        assert_eq!(r.next_int(10), 3);
        assert_eq!(r.next_int_unbounded(), -1360544799);
        assert_eq!(r.next_long(), 884324181205335268);
        assert_eq!(r.next_float(), 0.9420735);
        assert_eq!(r.next_double(), 0.27707849007413665);
        assert_eq!(r.next_int(7), 6);
        assert!(!r.next_bool());
        assert_eq!(
            JavaRandom::new(-123456789).next_double().to_bits(),
            4599720543886602408
        );
        assert_eq!(JavaRandom::new(7).next_float().to_bits(), 1060835095);
    }
}
