//! Java 8's `Math.sin`/`Math.cos` on HotSpot x86-64, which `WorldGenBigTree`
//! uses. Rust's libm disagrees in the last bit for ~2% of inputs, which can
//! move a branch by a block, so we reproduce Java instead:
//!
//! - `StrictMath` is fdlibm 5.3; ported below (`fdlibm_sin`/`fdlibm_cos`).
//! - HotSpot's intrinsic uses the x87 `fsin`/`fcos` for |x| < pi/4 and calls
//!   fdlibm otherwise (`MacroAssembler::trigfunc`). The x87 result is extended
//!   precision rounded to double; we emulate it with double-double arithmetic.
//!
//! Verified exhaustively against Java 8u51 (the JRE Mojang's launcher runs b1.7.3
//! on) for every input BigTree can produce; 23 hardware rounding outliers are
//! tabulated below. Newer JDKs use different intrinsics and give other results.

#[inline]
fn hi_word(x: f64) -> i32 {
    (x.to_bits() >> 32) as i32
}

#[inline]
fn with_hi_lo(hi: i32, lo: u32) -> f64 {
    f64::from_bits(((hi as u32 as u64) << 32) | lo as u64)
}

const S1: f64 = -1.66666666666666324348e-01;
const S2: f64 = 8.33333333332248946124e-03;
const S3: f64 = -1.98412698298579493134e-04;
const S4: f64 = 2.75573137070700676789e-06;
const S5: f64 = -2.50507602534068634195e-08;
const S6: f64 = 1.58969099521155010221e-10;

fn kernel_sin(x: f64, y: f64, iy: i32) -> f64 {
    let ix = hi_word(x) & 0x7fffffff;
    if ix < 0x3e400000 && x as i32 == 0 {
        return x;
    }
    let z = x * x;
    let v = z * x;
    let r = S2 + z * (S3 + z * (S4 + z * (S5 + z * S6)));
    if iy == 0 {
        x + v * (S1 + z * r)
    } else {
        x - ((z * (0.5 * y - v * r) - y) - v * S1)
    }
}

const C1: f64 = 4.16666666666666019037e-02;
const C2: f64 = -1.38888888888741095749e-03;
const C3: f64 = 2.48015872894767294178e-05;
const C4: f64 = -2.75573143513906633035e-07;
const C5: f64 = 2.08757232129817482790e-09;
const C6: f64 = -1.13596475577881948265e-11;

fn kernel_cos(x: f64, y: f64) -> f64 {
    let ix = hi_word(x) & 0x7fffffff;
    if ix < 0x3e400000 && x as i32 == 0 {
        return 1.0;
    }
    let z = x * x;
    let r = z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6)))));
    if ix < 0x3FD33333 {
        1.0 - (0.5 * z - (z * r - x * y))
    } else {
        let qx = if ix > 0x3fe90000 { 0.28125 } else { with_hi_lo(ix - 0x00200000, 0) };
        let hz = 0.5 * z - qx;
        let a = 1.0 - qx;
        a - (hz - (z * r - x * y))
    }
}

const INVPIO2: f64 = 6.36619772367581382433e-01;
const PIO2_1: f64 = 1.57079632673412561417e+00;
const PIO2_1T: f64 = 6.07710050650619224932e-11;
const PIO2_2: f64 = 6.07710050630396597660e-11;
const PIO2_2T: f64 = 2.02226624879595063154e-21;
const PIO2_3: f64 = 2.02226624871116645580e-21;
const PIO2_3T: f64 = 8.47842766036889956997e-32;

const NPIO2_HW: [i32; 32] = [
    0x3FF921FB, 0x400921FB, 0x4012D97C, 0x401921FB, 0x401F6A7A, 0x4022D97C, 0x4025FDBB, 0x402921FB, 0x402C463A,
    0x402F6A7A, 0x4031475C, 0x4032D97C, 0x40346B9C, 0x4035FDBB, 0x40378FDB, 0x403921FB, 0x403AB41B, 0x403C463A,
    0x403DD85A, 0x403F6A7A, 0x40407E4C, 0x4041475C, 0x4042106C, 0x4042D97C, 0x4043A28C, 0x40446B9C, 0x404534AC,
    0x4045FDBB, 0x4046C6CB, 0x40478FDB, 0x404858EB, 0x404921FB,
];

/// `__ieee754_rem_pio2` for |x| <= 2^19 * pi/2 (the large-argument path is not needed).
fn rem_pio2(x: f64) -> (i32, f64, f64) {
    let hx = hi_word(x);
    let ix = hx & 0x7fffffff;
    if ix <= 0x3fe921fb {
        return (0, x, 0.0);
    }
    if ix < 0x4002d97c {
        if hx > 0 {
            let mut z = x - PIO2_1;
            if ix != 0x3ff921fb {
                let y0 = z - PIO2_1T;
                return (1, y0, (z - y0) - PIO2_1T);
            }
            z -= PIO2_2;
            let y0 = z - PIO2_2T;
            return (1, y0, (z - y0) - PIO2_2T);
        }
        let mut z = x + PIO2_1;
        if ix != 0x3ff921fb {
            let y0 = z + PIO2_1T;
            return (-1, y0, (z - y0) + PIO2_1T);
        }
        z += PIO2_2;
        let y0 = z + PIO2_2T;
        return (-1, y0, (z - y0) + PIO2_2T);
    }
    assert!(ix <= 0x413921fb, "rem_pio2: large arguments not ported");
    let t = x.abs();
    let n = (t * INVPIO2 + 0.5) as i32;
    let fn_ = n as f64;
    let mut r = t - fn_ * PIO2_1;
    let mut w = fn_ * PIO2_1T;
    let mut y0;
    if n < 32 && ix != NPIO2_HW[(n - 1) as usize] {
        y0 = r - w;
    } else {
        let j = ix >> 20;
        y0 = r - w;
        let mut i = j - ((hi_word(y0) >> 20) & 0x7ff);
        if i > 16 {
            let t = r;
            w = fn_ * PIO2_2;
            r = t - w;
            w = fn_ * PIO2_2T - ((t - r) - w);
            y0 = r - w;
            i = j - ((hi_word(y0) >> 20) & 0x7ff);
            if i > 49 {
                let t = r;
                w = fn_ * PIO2_3;
                r = t - w;
                w = fn_ * PIO2_3T - ((t - r) - w);
                y0 = r - w;
            }
        }
    }
    let y1 = (r - y0) - w;
    if hx < 0 {
        (-n, -y0, -y1)
    } else {
        (n, y0, y1)
    }
}

/// `StrictMath.sin`
pub fn fdlibm_sin(x: f64) -> f64 {
    let ix = hi_word(x) & 0x7fffffff;
    if ix <= 0x3fe921fb {
        return kernel_sin(x, 0.0, 0);
    }
    if ix >= 0x7ff00000 {
        return x - x;
    }
    let (n, y0, y1) = rem_pio2(x);
    match n & 3 {
        0 => kernel_sin(y0, y1, 1),
        1 => kernel_cos(y0, y1),
        2 => -kernel_sin(y0, y1, 1),
        _ => -kernel_cos(y0, y1),
    }
}

/// `StrictMath.cos`
pub fn fdlibm_cos(x: f64) -> f64 {
    let ix = hi_word(x) & 0x7fffffff;
    if ix <= 0x3fe921fb {
        return kernel_cos(x, 0.0);
    }
    if ix >= 0x7ff00000 {
        return x - x;
    }
    let (n, y0, y1) = rem_pio2(x);
    match n & 3 {
        0 => kernel_cos(y0, y1),
        1 => -kernel_sin(y0, y1, 1),
        2 => -kernel_cos(y0, y1),
        _ => kernel_sin(y0, y1, 1),
    }
}

// ---------------------------------------------------------------- x87 emulation

/// Unevaluated sum hi + lo with |lo| <= ulp(hi)/2.
#[derive(Clone, Copy, Debug)]
struct DD {
    hi: f64,
    lo: f64,
}

#[inline]
fn two_sum(a: f64, b: f64) -> DD {
    let s = a + b;
    let bb = s - a;
    let e = (a - (s - bb)) + (b - bb);
    DD { hi: s, lo: e }
}

#[inline]
fn quick_two_sum(a: f64, b: f64) -> DD {
    let s = a + b;
    DD { hi: s, lo: b - (s - a) }
}

#[inline]
fn two_prod(a: f64, b: f64) -> DD {
    let p = a * b;
    DD { hi: p, lo: a.mul_add(b, -p) }
}

impl DD {
    fn from(x: f64) -> DD {
        DD { hi: x, lo: 0.0 }
    }
    fn add(self, o: DD) -> DD {
        let s = two_sum(self.hi, o.hi);
        let t = two_sum(self.lo, o.lo);
        let s = quick_two_sum(s.hi, s.lo + t.hi);
        quick_two_sum(s.hi, s.lo + t.lo)
    }
    fn mul(self, o: DD) -> DD {
        let p = two_prod(self.hi, o.hi);
        quick_two_sum(p.hi, p.lo + (self.hi * o.lo + self.lo * o.hi))
    }
    fn div_f(self, d: f64) -> DD {
        let q1 = self.hi / d;
        let r = self.add(two_prod(q1, d).neg());
        let q2 = r.hi / d;
        quick_two_sum(q1, q2)
    }
    fn neg(self) -> DD {
        DD { hi: -self.hi, lo: -self.lo }
    }
}

/// Taylor series in double-double; ~100 bits for |x| <= pi/4.
fn dd_sin_cos(x: f64, want_sin: bool) -> DD {
    let xx = DD::from(x);
    let x2 = xx.mul(xx);
    let (mut term, mut sum, mut k) = if want_sin { (xx, xx, 1.0) } else { (DD::from(1.0), DD::from(1.0), 0.0) };
    for _ in 0..30 {
        term = term.mul(x2).neg().div_f((k + 1.0) * (k + 2.0));
        k += 2.0;
        sum = sum.add(term);
        if term.hi.abs() < 1e-40 {
            break;
        }
    }
    sum
}

/// Round a double-double to a 64-bit significand (x87 extended), then to double
/// with ties-to-even: the double rounding an x87 `fsin` + `fstp m64` performs.
fn x87_round(v: DD) -> f64 {
    let v = quick_two_sum(v.hi, v.lo);
    let (hi, lo) = (v.hi, v.lo);
    if lo == 0.0 || hi == 0.0 {
        return hi;
    }
    let bits = hi.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i32;
    let ulp = f64::from_bits(((exp - 52) as u64) << 52); // ulp of hi (normal numbers)
    // Low part on the extended grid: multiples of ulp/2048, ties to even.
    let q = (lo / (ulp / 2048.0)).round_ties_even() * (ulp / 2048.0);
    let half = ulp / 2.0;
    if q.abs() < half {
        hi
    } else if q.abs() > half {
        // Can only happen when hi sits at a power of two (unequal ulps); defer to the sum.
        hi + q
    } else {
        // Exact tie on the double grid: round half to even.
        let other = if q > 0.0 { f64::from_bits(bits + 1) } else { f64::from_bits(bits - 1) };
        if bits & 1 == 0 {
            hi
        } else {
            other
        }
    }
}

const PI_4_HI_WORD: i32 = 0x3fe921fb;

// Inputs (of the 2^24 BigTree can produce) where this machine's x87 fsin/fcos,
// via Java 8u51 (Mojang's bundled JRE), is not the correctly rounded extended
// result. Hardware-specific: regenerate with `JMATH_TABLE=1 compare math` on
// another CPU vendor. Pairs are (input bits, Java result bits).
const X87_SIN_EXCEPTIONS: [(u64, u64); 18] = [
    (0x3fc1104ea8da3c21, 0x3fc103614fb5149f), // x=1.3331015820264816e-1
    (0x3fc226d753eb5b2d, 0x3fc217482ff6e512), // x=1.4181033704400062e-1
    (0x3fc55d68f6c3760c, 0x3fc5440c90414443), // x=1.6691314743995667e-1
    (0x3fc6e7c0d12599ed, 0x3fc6c88218beaf48), // x=1.789475461220741e-1
    (0x3fd2eca0557689ca, 0x3fd2a65633e4cfa3), // x=2.956925234413147e-1
    (0x3fd2f3a552c5197a, 0x3fd2ad0d1775d2d8), // x=2.9612095911979675e-1
    (0x3fd539468d5aaf79, 0x3fd4d63c98fa4557), // x=3.3162082485318184e-1
    (0x3fd842070fdfe32a, 0x3fd7ae663e827da5), // x=3.790300040268898e-1
    (0x3fd9247f88dfce31, 0x3fd88035cce9e7bd), // x=3.928526722586155e-1
    (0x3fdbe6356147ae14, 0x3fdb062413d260a7), // x=4.359258127212524e-1
    (0x3fdc44021f87bdcf, 0x3fdb5b0dfe68b338), // x=4.416508968269825e-1
    (0x3fdc45556a35e742, 0x3fdb5c40b9421a72), // x=4.417317902767658e-1
    (0x3fe0bc3d3281450f, 0x3fdff73228fc81c7), // x=5.229783998894691e-1
    (0x3fe10caa648b19a4, 0x3fe0411376889812), // x=5.327960933434963e-1
    (0x3fe2277f6722d0e5, 0x3fe1322d380a754a), // x=5.673214926123619e-1
    (0x3fe573abcee4e26d, 0x3fe3e17e22ddb90f), // x=6.703700104618072e-1
    (0x3fe8d8ef31b9b66f, 0x3fe66cbca12df388), // x=7.764812442016601e-1
    (0x3fe919088113be23, 0x3fe69a49a87a13e4), // x=7.843058129346371e-1
];
const X87_COS_EXCEPTIONS: [(u64, u64); 5] = [
    (0x3fd3b9e747d31b9b, 0x3fee7df2cd53546c), // x=3.082216454899311e-1
    (0x3fdca22eaf50b0f2, 0x3fecd9b4cfff1ba2), // x=4.473988258421421e-1
    (0x3fe35374807913e8, 0x3fea56e2684f99ea), // x=6.039373883473873e-1
    (0x3fe4b4c14ea18bd6, 0x3fe9880ac1037767), // x=6.470648322832584e-1
    (0x3fe640a82463736d, 0x3fe891d5dfb5f1c0), // x=6.953926764214039e-1
];

fn exception(table: &[(u64, u64)], x: f64) -> Option<f64> {
    let b = x.to_bits();
    table.iter().find(|e| e.0 == b).map(|e| f64::from_bits(e.1))
}

/// Java 8 HotSpot `Math.sin`.
pub fn java_sin(x: f64) -> f64 {
    if (hi_word(x) & 0x7fffffff) < PI_4_HI_WORD || x.abs() < std::f64::consts::FRAC_PI_4 {
        exception(&X87_SIN_EXCEPTIONS, x).unwrap_or_else(|| x87_round(dd_sin_cos(x, true)))
    } else {
        fdlibm_sin(x)
    }
}

/// Java 8 HotSpot `Math.cos`.
pub fn java_cos(x: f64) -> f64 {
    if (hi_word(x) & 0x7fffffff) < PI_4_HI_WORD || x.abs() < std::f64::consts::FRAC_PI_4 {
        exception(&X87_COS_EXCEPTIONS, x).unwrap_or_else(|| x87_round(dd_sin_cos(x, false)))
    } else {
        fdlibm_cos(x)
    }
}
