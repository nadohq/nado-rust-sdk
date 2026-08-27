use ethers::types::{I256, U256};
use eyre::{eyre, Result};

pub const ONE_X18: i128 = 1000000000000000000;

/// The health returned for a balance the risk system cannot price, matching
/// `INF` in contracts/common/Constants.sol. Large enough that any account
/// holding one fails every health check, small enough that callers can still
/// do arithmetic on the result without overflowing.
pub const INF: i128 = i128::MAX / 128;

/// Floor on the liquidation penalty for a non-spread position, matching
/// `MIN_NON_SPREAD_LIQ_PENALTY_X18` in contracts/common/Constants.sol.
pub const MIN_NON_SPREAD_LIQ_PENALTY_X18: i128 = ONE_X18 / 200;
pub const ONE_X6: i128 = 1_000_000;
pub const ONE_X12: i128 = 1_000_000_000_000;

const UONE: u128 = 1000000000000000000;

fn signed_to_unsigned(x: i128, y: i128) -> (u128, u128, i128) {
    if x >= 0 && y >= 0 {
        (x as u128, y as u128, 1)
    } else if x >= 0 && y < 0 {
        (x as u128, (-y) as u128, -1)
    } else if x < 0 && y >= 0 {
        ((-x) as u128, y as u128, -1)
    } else {
        ((-x) as u128, (-y) as u128, 1)
    }
}

// 128-bit division is a soft-float-style library call (~40-60ns) even for a constant
// divisor, and mul_x18/div-by-1e18 sits on every health/matching/fee path. This is the
// standard Granlund–Möller 2-by-1 division specialized to the constant 1e18: the first
// limb divides natively (64/64), the second uses a precomputed reciprocal — a handful
// of 64-bit multiplies, exact for every u128 input (verified against `/`/`%` in tests).
const UONE_U64: u64 = UONE as u64;
const UONE_SHIFT: u32 = UONE_U64.leading_zeros();
const UONE_NORM: u64 = UONE_U64 << UONE_SHIFT;
const UONE_RECIP: u64 = ((u128::MAX / UONE_NORM as u128) - (1u128 << 64)) as u64;

#[inline]
pub(crate) fn divmod_1e18(x: u128) -> (u128, u128) {
    let hi = (x >> 64) as u64;
    let lo = x as u64;

    let q_hi = hi / UONE_U64;
    let r_hi = hi % UONE_U64;

    // normalized 2-by-1 step for the remaining (r_hi, lo) dividend
    let u1 = (r_hi << UONE_SHIFT) | (lo >> (64 - UONE_SHIFT));
    let u0 = lo << UONE_SHIFT;

    let q = (UONE_RECIP as u128 * u1 as u128) + ((u1 as u128) << 64 | u0 as u128);
    let mut q1 = ((q >> 64) as u64).wrapping_add(1);
    let q0 = q as u64;
    let mut r = u0.wrapping_sub(q1.wrapping_mul(UONE_NORM));
    if r > q0 {
        q1 = q1.wrapping_sub(1);
        r = r.wrapping_add(UONE_NORM);
    }
    if r >= UONE_NORM {
        q1 += 1;
        r -= UONE_NORM;
    }

    ((q_hi as u128) << 64 | q1 as u128, (r >> UONE_SHIFT) as u128)
}

pub fn mul_x18(x: i128, y: i128) -> i128 {
    let (mut x, mut y, sign) = signed_to_unsigned(x, y);
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    (if y < UONE {
        divmod_1e18(x * y).0
    } else if x < UONE {
        let (c, d) = divmod_1e18(y);
        x * c + divmod_1e18(x * d).0
    } else {
        let (a, b) = divmod_1e18(x);
        let (c, d) = divmod_1e18(y);
        a * c * UONE + a * d + b * c + divmod_1e18(b * d).0
    } as i128)
        * sign
}

// TODO: replacing mul_x18 with fmul_x18 can result in 1.5-2x speedup
pub fn fmul_x18(x: i128, y: i128) -> i128 {
    ((x as f64 * y as f64) / 1e18) as i128
}

pub fn div_x18(x: i128, y: i128) -> i128 {
    let (mut x, y, sign) = signed_to_unsigned(x, y);
    let mut ret = 0;
    if x >= y {
        ret += x / y * UONE;
        x %= y;
    }
    if x <= UONE {
        ret += x * UONE / y;
    } else {
        ret += (U256::from(x) * U256::from(UONE) / U256::from(y)).low_u128()
    }
    (ret as i128) * sign
}

pub fn pow_x18(x: i128, y: i128) -> i128 {
    let xf = x18_to_f64(x);
    let yf = x18_to_f64(y);
    let resultf = xf.powf(yf);
    let mut result = (resultf.trunc() as i128) * ONE_X18;
    result += (resultf.fract() * 1e18f64) as i128;
    result
}

pub fn sqrt_x18(x: i128) -> i128 {
    pow_x18(x, ONE_X18 / 2)
}

pub fn mul_div_x18(x: i128, y: i128, z: i128) -> i128 {
    (I256::from(x) * I256::from(y) / I256::from(z)).low_i128()
}

pub fn x18_to_f64_mil(x: i128) -> f64 {
    let x = x / 1_000_000;
    x18_to_f64(x)
}

// aarch64 has no native i128→f64 conversion, so `x as f64` calls a soft-float routine;
// two native u64 converts plus one combine are ~10× cheaper and within one ulp. Split on
// the magnitude, not two's-complement limbs: a signed hi limb cancels against lo for
// small negatives (-1 would become -2^64 + round(2^64 - 1) = 0.0, losing the sign)
#[inline]
pub(crate) fn i128_to_f64(x: i128) -> f64 {
    let mag = x.unsigned_abs();
    let hi = (mag >> 64) as u64 as f64;
    let lo = mag as u64 as f64;
    let abs = hi * 18_446_744_073_709_551_616.0 + lo;
    if x < 0 {
        -abs
    } else {
        abs
    }
}

// quotient/remainder form, not `i128_to_f64(x) * 1e-18`: fl(1e-18) is inexact, which
// would put an ulp of error on whole-token values (7 tokens → 7.000000000000001) that
// the pre-optimization implementation converted exactly. 1e18 is exactly representable,
// so dividing the sub-token remainder keeps this bit-identical to the old `/` + `%` form
pub fn x18_to_f64(x: i128) -> f64 {
    let (q, r) = divmod_1e18(x.unsigned_abs());
    let abs = i128_to_f64(q as i128) + (r as u64) as f64 / 1e18;
    if x < 0 {
        -abs
    } else {
        abs
    }
}

pub fn f64_to_x18(x: f64) -> i128 {
    let mut result = (x.trunc() as i128) * ONE_X18;
    result += (x.fract() * 1e18) as i128;
    result
}

pub fn split_i256(x: I256) -> (i128, i128) {
    let base = I256::from(2).pow(127);
    ((x / base).as_i128(), (x % base).as_i128())
}

pub fn merge_i128(x: i128, y: i128) -> I256 {
    let base = I256::from(2).pow(127);
    I256::from(x) * base + I256::from(y)
}

pub fn i256_to_f64(x: I256) -> f64 {
    let (high, low) = split_i256(x);
    x18_to_f64(high) * (2.0_f64).powi(127) + x18_to_f64(low)
}

pub fn to_u128_x18(x: u128) -> u128 {
    x * (ONE_X18 as u128)
}

pub fn to_i128_x18(x: i128) -> i128 {
    x * ONE_X18
}

pub fn to_i128_fp(x: f64) -> i128 {
    (x * 10.0_f64.powi(9)) as i128 * 1000000000
}

pub fn to_i32_fp(x: f64) -> i32 {
    (x * 10.0_f64.powi(9)) as i32
}

pub fn x18_to_x9(x: i128) -> i32 {
    (x / 1_000_000_000) as i32
}

pub fn str_to_x18(s: &str) -> i128 {
    let parts: Vec<&str> = s.split('.').collect();
    let whole = parts[0].parse::<i128>().unwrap_or(0);
    let frac = if parts.len() > 1 {
        let frac_part = parts[1];
        let frac_str = if frac_part.len() <= 18 {
            format!("{frac_part:0<18}")
        } else {
            frac_part[..18].to_string()
        };
        frac_str.parse::<i128>().unwrap_or(0)
    } else {
        0
    };
    whole * ONE_X18 + frac
}

pub fn to_u128_x6(x: u128) -> u128 {
    x * 1000000
}

pub fn to_i128_x6(x: i128) -> i128 {
    x * 1000000
}

pub fn fexp_x18(mut x: i128, y: i128) -> i128 {
    assert!(y >= 0);
    let mut i = 1;
    let mut ret = to_i128_x18(1);
    while i <= y {
        if i & y != 0 {
            ret = mul_x18(ret, x);
        }
        x = mul_x18(x, x);
        i <<= 1;
    }
    ret
}

pub fn fexp(mut x: i128, y: i128) -> i128 {
    assert!(y >= 0);
    let mut i = 1;
    let mut ret = 1;
    while i <= y {
        if i & y != 0 {
            ret *= x;
        }
        x *= x;
        i <<= 1;
    }
    ret
}

pub fn check_diff_gt_threshold_x18(left_x18: i128, right_x18: i128, threshold: f64) -> bool {
    let diff = (left_x18 - right_x18).abs();
    let percent_diff = div_x18(diff, right_x18);
    let percent_threshold: f64 = threshold * 1e18f64;
    percent_diff > percent_threshold as i128
}

pub fn check_within_range_x18(
    left_x18: i128,
    right_x18: i128,
    threshold_lower: f64,
    threshold_upper: f64,
) -> bool {
    let percent = div_x18(left_x18, right_x18);
    let percent_threshold_lower: f64 = threshold_lower * 1e18f64;
    let percent_threshold_upper: f64 = threshold_upper * 1e18f64;
    (percent > percent_threshold_lower as i128) && (percent < percent_threshold_upper as i128)
}

pub trait TryMath {
    fn try_add(self, v: i128) -> Result<i128>;
    fn try_div(self, v: i128) -> Result<i128>;
    fn try_mul(self, v: i128) -> Result<i128>;
    fn try_sub(self, v: i128) -> Result<i128>;
    fn try_rem(self, v: i128) -> Result<i128>;
    fn try_mul_x18(self, v: i128) -> Result<i128>;
    fn try_div_x18(self, v: i128) -> Result<i128>;
    fn try_sqrt_x18(self) -> Result<i128>;
}

impl TryMath for i128 {
    fn try_add(self, v: i128) -> Result<i128> {
        self.checked_add(v).ok_or(eyre!("Overflow: add"))
    }

    fn try_div(self, v: i128) -> Result<i128> {
        self.checked_div(v).ok_or(eyre!("Overflow: div"))
    }

    fn try_mul(self, v: i128) -> Result<i128> {
        self.checked_mul(v).ok_or(eyre!("Overflow: mul"))
    }

    fn try_sub(self, v: i128) -> Result<i128> {
        self.checked_sub(v).ok_or(eyre!("Overflow: sub"))
    }

    fn try_rem(self, v: i128) -> Result<i128> {
        self.checked_rem(v).ok_or(eyre!("Overflow: rem"))
    }

    fn try_mul_x18(self, v: i128) -> Result<i128> {
        Ok((I256::from(self) * I256::from(v) / I256::exp10(18)).as_i128())
    }

    fn try_div_x18(self, v: i128) -> Result<i128> {
        Ok((I256::from(self) * I256::exp10(18) / I256::from(v)).as_i128())
    }

    fn try_sqrt_x18(self) -> Result<i128> {
        let mut hi = 1;
        while hi.try_mul_x18(hi)? < self {
            hi = hi.try_mul(2)?;
        }
        let mut lo = hi.try_div(2)?;
        while lo < hi {
            let mid = lo.try_add(hi)?.try_div(2)?;
            if mid.try_mul_x18(mid)? < self {
                lo = mid.try_add(1)?;
            } else {
                hi = mid;
            }
        }
        Ok(lo)
    }
}

pub fn lp_value(balance: i128, x: i128, y: i128, supply: i128, price: i128) -> i128 {
    if supply == 0 {
        0
    } else {
        let pool_total_value = mul_x18(x, price) + y;
        mul_div_x18(balance, pool_total_value, supply)
    }
}

pub fn trunc(value: i128, increment: i128) -> i128 {
    value - value % increment
}

const ROUND_INCREMENT: i128 = 100_000_000;
const ROUND_THRESHOLD: i128 = 1_000_000;
pub fn spot_round(mut value: i128) -> i128 {
    let negative = value < 0;
    if negative {
        value = -value;
    }
    let m = value % ROUND_INCREMENT;
    if m < ROUND_THRESHOLD {
        value -= m;
    } else if m > ROUND_INCREMENT - ROUND_THRESHOLD {
        value += ROUND_INCREMENT - m;
    }
    if negative {
        value = -value;
    }
    value
}

pub fn expo_to_x18(mut value: i128, expo: i32) -> i128 {
    let shift = 18 + expo;
    if shift >= 0 {
        for _ in 0..shift {
            value = value.saturating_mul(10);
        }
    } else {
        for _ in 0..-shift {
            value /= 10;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    // deterministic splitmix64 so the sweep is reproducible without a rand dependency
    fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    #[test]
    fn divmod_1e18_matches_native_division() {
        let edges = [
            0u128,
            1,
            UONE - 1,
            UONE,
            UONE + 1,
            u64::MAX as u128,
            u64::MAX as u128 + 1,
            UONE * UONE,
            UONE * UONE - 1,
            UONE * UONE + 1,
            i128::MAX as u128,
            u128::MAX,
            u128::MAX - 1,
        ];
        for x in edges {
            assert_eq!(divmod_1e18(x), (x / UONE, x % UONE), "edge {x}");
        }

        let mut state = 0xDEADBEEFu64;
        for _ in 0..5_000_000 {
            let x = (splitmix64(&mut state) as u128) << 64 | splitmix64(&mut state) as u128;
            // vary magnitude so small values are covered too
            let x = x >> (splitmix64(&mut state) % 128);
            assert_eq!(divmod_1e18(x), (x / UONE, x % UONE), "random {x}");
        }
    }

    #[test]
    fn i128_to_f64_matches_native_conversion() {
        // `x as f64` is correctly rounded; the split conversion may double-round, so
        // allow one ulp — but small magnitudes must be exact (this is what the old
        // two's-complement split got wrong: i128_to_f64(-1) returned 0.0)
        fn check(x: i128) {
            let got = i128_to_f64(x);
            let want = x as f64;
            if x.unsigned_abs() <= 1 << 53 {
                assert_eq!(got, want, "x={x}");
            } else {
                assert!(
                    (got - want).abs() <= want.abs() * f64::EPSILON,
                    "x={x} got={got} want={want}"
                );
            }
        }

        let edges = [
            0i128,
            1,
            -1,
            -1000,
            -123456,
            (1 << 53) - 1,
            1 << 53,
            -(1 << 53),
            u64::MAX as i128,
            -(u64::MAX as i128),
            (1 << 64) + 1,
            -(1 << 64) - 1,
            i128::MAX,
            i128::MIN,
            i128::MIN + 1,
        ];
        for x in edges {
            check(x);
        }

        let mut state = 0xF64BEEFu64;
        for _ in 0..2_000_000 {
            let x = (splitmix64(&mut state) as u128) << 64 | splitmix64(&mut state) as u128;
            let x = (x >> (splitmix64(&mut state) % 128)) as i128;
            check(x);
            check(x.wrapping_neg());
        }
    }

    #[test]
    fn mul_x18_matches_reference() {
        let mut state = 0xC0FFEEu64;
        for _ in 0..2_000_000 {
            // keep magnitudes within what the reference computes without overflow
            let x = (splitmix64(&mut state) as u128) << 64 | splitmix64(&mut state) as u128;
            let y = (splitmix64(&mut state) as u128) << 64 | splitmix64(&mut state) as u128;
            let x = (x >> (splitmix64(&mut state) % 68 + 60)) as i128
                * if splitmix64(&mut state).is_multiple_of(2) {
                    1
                } else {
                    -1
                };
            let y = (y >> (splitmix64(&mut state) % 68 + 60)) as i128
                * if splitmix64(&mut state).is_multiple_of(2) {
                    1
                } else {
                    -1
                };
            // mul_div_x18 widens through I256 with the same truncate-toward-zero
            // semantics — a structurally independent oracle, unlike the old limb-split
            // reference that shared its decomposition with mul_x18 itself
            assert_eq!(mul_x18(x, y), mul_div_x18(x, y, ONE_X18), "x={x} y={y}");
        }
    }

    #[test]
    fn x18_to_f64_matches_divrem_reference() {
        // the reference is the pre-optimization implementation; whole-token values in
        // particular must convert exactly (7e18 → 7.0, not 7.000000000000001)
        fn reference(x: i128) -> f64 {
            (x / ONE_X18) as f64 + (x % ONE_X18) as f64 / 1e18
        }
        fn check(x: i128) {
            let got = x18_to_f64(x);
            let want = reference(x);
            if (x / ONE_X18).unsigned_abs() <= 1 << 53 {
                assert_eq!(got, want, "x={x}");
            } else {
                assert!(
                    (got - want).abs() <= want.abs() * f64::EPSILON,
                    "x={x} got={got} want={want}"
                );
            }
        }

        let edges = [
            0i128,
            1,
            -1,
            ONE_X18,
            -ONE_X18,
            7 * ONE_X18,
            -7 * ONE_X18,
            65_432_100_000_000_000_000_000,
            123_456_789 * ONE_X18,
            ONE_X18 - 1,
            ONE_X18 + 1,
            i128::MAX,
            i128::MIN,
        ];
        for x in edges {
            check(x);
        }

        let mut state = 0x18F64u64;
        for _ in 0..2_000_000 {
            let x = (splitmix64(&mut state) as u128) << 64 | splitmix64(&mut state) as u128;
            let x = (x >> (splitmix64(&mut state) % 128)) as i128;
            check(x);
            check(x.wrapping_neg());
        }
    }
}
