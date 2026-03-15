use std::fmt::Debug;

use rand::Rng;

use crate::gen::Gen;
use crate::shrink::empty_shrinker;

/// Trait for generating arbitrary values and shrinking them.
///
/// Types implementing `Arbitrary` can produce random instances via
/// [`arbitrary`](Arbitrary::arbitrary) and optionally provide a
/// [`shrink`](Arbitrary::shrink) method to reduce failing inputs.
pub trait Arbitrary: Clone + Debug + 'static {
    /// Generate an arbitrary value using the given generator.
    fn arbitrary(g: &mut Gen) -> Self;

    /// Return an iterator of values that are "smaller" than `self`.
    ///
    /// Used to find minimal counterexamples when a property fails.
    /// The default implementation returns an empty iterator.
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        empty_shrinker()
    }
}

// -- bool --

impl Arbitrary for bool {
    fn arbitrary(g: &mut Gen) -> Self {
        g.rng().gen()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        if *self {
            Box::new(std::iter::once(false))
        } else {
            empty_shrinker()
        }
    }
}

// -- unsigned integers --

macro_rules! arbitrary_unsigned {
    ($ty:ty) => {
        impl Arbitrary for $ty {
            fn arbitrary(g: &mut Gen) -> Self {
                let s = g.size();
                if s == 0 {
                    return 0;
                }
                let upper = if s as u128 >= <$ty>::MAX as u128 {
                    <$ty>::MAX
                } else {
                    s as $ty
                };
                g.gen_range(0..=upper)
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                let val = *self;
                if val == 0 {
                    return empty_shrinker();
                }
                let mut candidates = Vec::new();
                candidates.push(0);
                let mut diff = val;
                loop {
                    diff /= 2;
                    if diff == 0 {
                        break;
                    }
                    candidates.push(val - diff);
                }
                Box::new(candidates.into_iter())
            }
        }
    };
}

arbitrary_unsigned!(u8);
arbitrary_unsigned!(u16);
arbitrary_unsigned!(u32);
arbitrary_unsigned!(u64);
arbitrary_unsigned!(u128);
arbitrary_unsigned!(usize);

// -- signed integers --
// Uses checked_neg to fix quickcheck's integer negation overflow bug on MIN values.

macro_rules! arbitrary_signed {
    ($ty:ty) => {
        impl Arbitrary for $ty {
            fn arbitrary(g: &mut Gen) -> Self {
                let s = g.size();
                if s == 0 {
                    return 0;
                }
                let upper = if s as u128 >= <$ty>::MAX as u128 {
                    <$ty>::MAX
                } else {
                    s as $ty
                };
                let lower = if upper == <$ty>::MAX {
                    <$ty>::MIN
                } else {
                    -upper
                };
                g.gen_range(lower..=upper)
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                let val = *self;
                if val == 0 {
                    return empty_shrinker();
                }
                let mut candidates = Vec::new();
                candidates.push(0);

                // For negative values, try the positive counterpart.
                // checked_neg returns None for MIN, preventing overflow.
                if val < 0 {
                    if let Some(pos) = val.checked_neg() {
                        candidates.push(pos);
                    }
                }

                // Halve toward 0
                let mut diff = val;
                loop {
                    diff /= 2;
                    if diff == 0 {
                        break;
                    }
                    let candidate = val - diff;
                    if candidate != 0 {
                        candidates.push(candidate);
                    }
                }

                Box::new(candidates.into_iter())
            }
        }
    };
}

arbitrary_signed!(i8);
arbitrary_signed!(i16);
arbitrary_signed!(i32);
arbitrary_signed!(i64);
arbitrary_signed!(i128);
arbitrary_signed!(isize);

// -- floating point --

impl Arbitrary for f32 {
    fn arbitrary(g: &mut Gen) -> Self {
        let s = g.size() as f32;
        g.rng().gen::<f32>() * s * 2.0 - s
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let val = *self;
        if val == 0.0 {
            return empty_shrinker();
        }
        let mut candidates = vec![0.0f32];
        if val < 0.0 {
            candidates.push(-val);
        }
        let mut x = val;
        for _ in 0..30 {
            x /= 2.0;
            if x.abs() < f32::EPSILON {
                break;
            }
            candidates.push(val - x);
        }
        Box::new(candidates.into_iter())
    }
}

impl Arbitrary for f64 {
    fn arbitrary(g: &mut Gen) -> Self {
        let s = g.size() as f64;
        g.rng().gen::<f64>() * s * 2.0 - s
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let val = *self;
        if val == 0.0 {
            return empty_shrinker();
        }
        let mut candidates = vec![0.0f64];
        if val < 0.0 {
            candidates.push(-val);
        }
        let mut x = val;
        for _ in 0..60 {
            x /= 2.0;
            if x.abs() < f64::EPSILON {
                break;
            }
            candidates.push(val - x);
        }
        Box::new(candidates.into_iter())
    }
}

// -- char --

impl Arbitrary for char {
    fn arbitrary(g: &mut Gen) -> Self {
        let s = g.size();
        if s <= 127 {
            let cp = g.gen_range(0u32..=s as u32).min(127);
            char::from_u32(cp).unwrap_or('a')
        } else {
            loop {
                let cp = g.gen_range(0u32..=0x10FFFF);
                if let Some(c) = char::from_u32(cp) {
                    return c;
                }
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let c = *self;
        if c == 'a' {
            return empty_shrinker();
        }
        let mut candidates = Vec::new();
        candidates.push('a');
        if c.is_uppercase() {
            for lower in c.to_lowercase() {
                if lower != 'a' {
                    candidates.push(lower);
                }
            }
        }
        let target = 'a' as u32;
        let val = c as u32;
        if val > target {
            let mut diff = val - target;
            while diff > 0 {
                diff /= 2;
                if diff == 0 {
                    break;
                }
                if let Some(ch) = char::from_u32(val - diff) {
                    if ch != 'a' {
                        candidates.push(ch);
                    }
                }
            }
        }
        Box::new(candidates.into_iter())
    }
}

// -- unit --

impl Arbitrary for () {
    fn arbitrary(_g: &mut Gen) -> Self {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_gen() -> Gen {
        Gen::from_seed(100, 12345)
    }

    #[test]
    fn test_bool_arbitrary() {
        let mut g = test_gen();
        let mut seen_true = false;
        let mut seen_false = false;
        for _ in 0..100 {
            let b = bool::arbitrary(&mut g);
            if b {
                seen_true = true;
            } else {
                seen_false = true;
            }
        }
        assert!(seen_true && seen_false);
    }

    #[test]
    fn test_bool_shrink() {
        let shrunk: Vec<_> = true.shrink().collect();
        assert_eq!(shrunk, vec![false]);
        let shrunk: Vec<_> = false.shrink().collect();
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_u8_arbitrary_within_size() {
        let mut g = Gen::from_seed(10, 42);
        for _ in 0..200 {
            let v = u8::arbitrary(&mut g);
            assert!(v <= 10, "u8 should be <= size (10), got {}", v);
        }
    }

    #[test]
    fn test_u32_arbitrary_within_size() {
        let mut g = Gen::from_seed(50, 42);
        for _ in 0..200 {
            let v = u32::arbitrary(&mut g);
            assert!(v <= 50, "u32 should be <= size (50), got {}", v);
        }
    }

    #[test]
    fn test_i32_arbitrary_within_size() {
        let mut g = Gen::from_seed(50, 42);
        for _ in 0..200 {
            let v = i32::arbitrary(&mut g);
            assert!((-50..=50).contains(&v), "i32 should be in [-50, 50], got {}", v);
        }
    }

    #[test]
    fn test_unsigned_shrink_toward_zero() {
        let shrunk: Vec<u32> = 100u32.shrink().collect();
        assert!(!shrunk.is_empty());
        assert_eq!(shrunk[0], 0);
        for v in &shrunk {
            assert!(*v < 100);
        }
    }

    #[test]
    fn test_signed_shrink_toward_zero() {
        let shrunk: Vec<i32> = (-100i32).shrink().collect();
        assert!(!shrunk.is_empty());
        assert_eq!(shrunk[0], 0);
        assert!(shrunk.contains(&100));
    }

    #[test]
    fn test_signed_shrink_min_no_overflow() {
        // Regression: quickcheck panics on i8::MIN.shrink() due to negation overflow
        let shrunk: Vec<i8> = i8::MIN.shrink().collect();
        assert!(!shrunk.is_empty());
        assert_eq!(shrunk[0], 0);
    }

    #[test]
    fn test_i128_min_shrink_no_overflow() {
        let shrunk: Vec<i128> = i128::MIN.shrink().collect();
        assert!(!shrunk.is_empty());
        assert_eq!(shrunk[0], 0);
    }

    #[test]
    fn test_zero_no_shrink() {
        let shrunk: Vec<u32> = 0u32.shrink().collect();
        assert!(shrunk.is_empty());
        let shrunk: Vec<i32> = 0i32.shrink().collect();
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_f64_arbitrary() {
        let mut g = Gen::from_seed(100, 42);
        for _ in 0..200 {
            let v = f64::arbitrary(&mut g);
            assert!(v >= -100.0 && v <= 100.0, "f64 should be in [-100, 100], got {}", v);
        }
    }

    #[test]
    fn test_f64_shrink() {
        let shrunk: Vec<f64> = 42.0f64.shrink().collect();
        assert!(!shrunk.is_empty());
        assert_eq!(shrunk[0], 0.0);
    }

    #[test]
    fn test_f32_shrink_negative() {
        let shrunk: Vec<f32> = (-10.0f32).shrink().collect();
        assert!(shrunk.contains(&0.0));
        assert!(shrunk.contains(&10.0));
    }

    #[test]
    fn test_char_arbitrary() {
        let mut g = Gen::from_seed(100, 42);
        for _ in 0..200 {
            let c = char::arbitrary(&mut g);
            assert!(c as u32 <= 0x10FFFF);
        }
    }

    #[test]
    fn test_char_shrink() {
        let shrunk: Vec<char> = 'z'.shrink().collect();
        assert!(shrunk.contains(&'a'));

        let shrunk: Vec<char> = 'a'.shrink().collect();
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_unit_arbitrary() {
        let mut g = test_gen();
        let _: () = Arbitrary::arbitrary(&mut g);
    }
}
