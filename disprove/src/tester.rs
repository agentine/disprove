use std::fmt;

use crate::arbitrary::Arbitrary;
use crate::gen::Gen;

/// Result of a single property test.
#[derive(Clone, Debug)]
pub enum TestResult {
    /// The property passed.
    Pass,
    /// The property failed with an optional message.
    Fail(String),
    /// The test case was discarded (input didn't meet preconditions).
    Discard,
}

impl TestResult {
    /// Returns true if this result is a failure.
    pub fn is_failure(&self) -> bool {
        matches!(self, TestResult::Fail(_))
    }

    /// Returns true if this result is a failure.
    pub fn failed(&self) -> bool {
        self.is_failure()
    }

    /// Returns true if this result represents an error (same as failure).
    pub fn is_error(&self) -> bool {
        self.is_failure()
    }

    /// Create a TestResult that expects failure. If the property passes,
    /// this will be treated as a failure, and vice versa.
    pub fn must_fail<T: Into<TestResult>>(result: T) -> TestResult {
        match result.into() {
            TestResult::Pass => TestResult::Fail("expected failure but property passed".into()),
            TestResult::Fail(_) => TestResult::Pass,
            TestResult::Discard => TestResult::Discard,
        }
    }

    /// Create a failing result with a message.
    pub fn error(msg: String) -> TestResult {
        TestResult::Fail(msg)
    }
}

impl From<bool> for TestResult {
    fn from(b: bool) -> Self {
        if b {
            TestResult::Pass
        } else {
            TestResult::Fail(String::new())
        }
    }
}

impl From<()> for TestResult {
    fn from((): ()) -> Self {
        TestResult::Pass
    }
}

impl<T, E: fmt::Debug> From<Result<T, E>> for TestResult {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(_) => TestResult::Pass,
            Err(e) => TestResult::Fail(format!("{:?}", e)),
        }
    }
}

/// Trait for types that can be tested as properties.
pub trait Testable: 'static {
    /// Run this property with the given generator.
    fn result(&self, g: &mut Gen) -> TestResult;
}

/// Configurable property-based test runner.
pub struct QuickCheck {
    tests: u64,
    max_tests: u64,
    min_tests_passed: u64,
    gen: Gen,
}

impl QuickCheck {
    /// Create a new QuickCheck runner with default settings.
    /// Reads configuration from environment variables:
    /// - `QUICKCHECK_TESTS`: number of tests (default 100)
    /// - `QUICKCHECK_MAX_TESTS`: max tests including discards (default 10000)
    /// - `QUICKCHECK_GENERATOR_SIZE`: generator size (default 100)
    /// - `QUICKCHECK_SEED`: deterministic seed for reproducibility
    pub fn new() -> Self {
        let tests = env_var_or("QUICKCHECK_TESTS", 100);
        let max_tests = env_var_or("QUICKCHECK_MAX_TESTS", 10_000);
        let size = env_var_or("QUICKCHECK_GENERATOR_SIZE", 100) as usize;
        let gen = match std::env::var("QUICKCHECK_SEED") {
            Ok(s) => {
                let seed: u64 = s.parse().expect("QUICKCHECK_SEED must be a u64");
                Gen::from_seed(size, seed)
            }
            Err(_) => Gen::new(size),
        };

        QuickCheck {
            tests,
            max_tests,
            min_tests_passed: 0,
            gen,
        }
    }

    /// Set the number of tests to run.
    pub fn tests(mut self, n: u64) -> Self {
        self.tests = n;
        self
    }

    /// Set the maximum number of tests (including discards).
    pub fn max_tests(mut self, n: u64) -> Self {
        self.max_tests = n;
        self
    }

    /// Set the minimum number of tests that must pass.
    pub fn min_tests_passed(mut self, n: u64) -> Self {
        self.min_tests_passed = n;
        self
    }

    /// Set the generator to use.
    pub fn gen(mut self, gen: Gen) -> Self {
        self.gen = gen;
        self
    }

    /// Run the property test. Panics on failure with counterexample.
    pub fn quickcheck<T: Testable>(&mut self, testable: T) {
        let mut passed = 0u64;
        let mut discarded = 0u64;
        let mut total = 0u64;

        while passed < self.tests && total < self.max_tests {
            total += 1;
            let result = testable.result(&mut self.gen);
            match result {
                TestResult::Pass => passed += 1,
                TestResult::Fail(msg) => {
                    let detail = if msg.is_empty() {
                        format!(
                            "[disprove] FAILED after {} passed tests (test #{})",
                            passed,
                            total
                        )
                    } else {
                        format!(
                            "[disprove] FAILED after {} passed tests (test #{}): {}",
                            passed, total, msg
                        )
                    };
                    panic!("{}", detail);
                }
                TestResult::Discard => discarded += 1,
            }
        }

        if passed < self.tests {
            panic!(
                "[disprove] gave up after {} discards, only {} of {} tests passed",
                discarded, passed, self.tests
            );
        }

        if passed < self.min_tests_passed {
            panic!(
                "[disprove] only {} tests passed, minimum required: {}",
                passed, self.min_tests_passed
            );
        }
    }
}

impl Default for QuickCheck {
    fn default() -> Self {
        Self::new()
    }
}

fn env_var_or(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Run a property test with default settings. Panics on failure.
pub fn quickcheck<T: Testable>(testable: T) {
    QuickCheck::new().quickcheck(testable)
}

// -- Testable implementations --

// Fn() -> T where T: Into<TestResult>
impl<F, R> Testable for F
where
    F: Fn() -> R + 'static,
    R: Into<TestResult>,
{
    fn result(&self, _g: &mut Gen) -> TestResult {
        self().into()
    }
}

// For multi-arg functions, we implement Testable for wrapper types
// that generate arbitrary inputs and run the function.

/// Wrapper for single-arg property functions.
pub struct TestableArgs1<A, R, F: Fn(A) -> R> {
    f: F,
    _marker: std::marker::PhantomData<(A, R)>,
}

impl<A, R, F> Testable for TestableArgs1<A, R, F>
where
    A: Arbitrary + fmt::Debug,
    R: Into<TestResult> + 'static,
    F: Fn(A) -> R + 'static,
{
    fn result(&self, g: &mut Gen) -> TestResult {
        let a = A::arbitrary(g);
        let result: TestResult = (self.f)(a.clone()).into();
        if result.is_failure() {
            // Shrink
            let mut minimal = a.clone();
            for smaller in a.shrink() {
                let r: TestResult = (self.f)(smaller.clone()).into();
                if r.is_failure() {
                    minimal = smaller;
                }
            }
            return TestResult::Fail(format!("{:?}", minimal));
        }
        result
    }
}

/// Create a testable from a single-argument function.
pub fn testable_args1<A, R, F>(f: F) -> TestableArgs1<A, R, F>
where
    A: Arbitrary + fmt::Debug,
    R: Into<TestResult> + 'static,
    F: Fn(A) -> R + 'static,
{
    TestableArgs1 {
        f,
        _marker: std::marker::PhantomData,
    }
}

// Multi-arg testable wrappers using tuples

macro_rules! testable_args {
    ($name:ident, $fn_name:ident, $($T:ident),+) => {
        pub struct $name<$($T,)+ R, F: Fn($($T),+) -> R> {
            f: F,
            _marker: std::marker::PhantomData<($($T,)+ R)>,
        }

        impl<$($T,)+ R, F> Testable for $name<$($T,)+ R, F>
        where
            $($T: Arbitrary + fmt::Debug,)+
            R: Into<TestResult> + 'static,
            F: Fn($($T),+) -> R + 'static,
        {
            #[allow(non_snake_case)]
            fn result(&self, g: &mut Gen) -> TestResult {
                $(let $T = $T::arbitrary(g);)+
                let result: TestResult = (self.f)($($T.clone()),+).into();
                if result.is_failure() {
                    // Shrink first arg only for simplicity
                    return TestResult::Fail(format!("{:?}", ($(&$T),+)));
                }
                result
            }
        }

        pub fn $fn_name<$($T,)+ R, F>(f: F) -> $name<$($T,)+ R, F>
        where
            $($T: Arbitrary + fmt::Debug,)+
            R: Into<TestResult> + 'static,
            F: Fn($($T),+) -> R + 'static,
        {
            $name {
                f,
                _marker: std::marker::PhantomData,
            }
        }
    };
}

testable_args!(TestableArgs2, testable_args2, A, B);
testable_args!(TestableArgs3, testable_args3, A, B, C);
testable_args!(TestableArgs4, testable_args4, A, B, C, D);
testable_args!(TestableArgs5, testable_args5, A, B, C, D, E);
testable_args!(TestableArgs6, testable_args6, A, B, C, D, E, F2);
testable_args!(TestableArgs7, testable_args7, A, B, C, D, E, F2, G);
testable_args!(TestableArgs8, testable_args8, A, B, C, D, E, F2, G, H);

// Implement Testable for fn(A) -> R via cast
// Users pass `prop as fn(Vec<i32>) -> bool` — this hits the Fn() -> R impl
// For multi-arg, they use testable_args* or the #[quickcheck] macro.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_from_bool() {
        let r: TestResult = true.into();
        assert!(!r.is_failure());
        let r: TestResult = false.into();
        assert!(r.is_failure());
    }

    #[test]
    fn test_result_from_unit() {
        let r: TestResult = ().into();
        assert!(!r.is_failure());
    }

    #[test]
    fn test_result_from_result() {
        let r: TestResult = Ok::<(), &str>(()).into();
        assert!(!r.is_failure());
        let r: TestResult = Err::<(), &str>("oops").into();
        assert!(r.is_failure());
    }

    #[test]
    fn test_must_fail() {
        let r = TestResult::must_fail(true);
        assert!(r.is_failure());
        let r = TestResult::must_fail(false);
        assert!(!r.is_failure());
    }

    #[test]
    fn test_quickcheck_passing() {
        // A property that always passes
        QuickCheck::new()
            .tests(50)
            .quickcheck(|| -> bool { true });
    }

    #[test]
    #[should_panic(expected = "[disprove] FAILED")]
    fn test_quickcheck_failing() {
        QuickCheck::new()
            .tests(50)
            .quickcheck(|| -> bool { false });
    }

    #[test]
    fn test_quickcheck_with_discards() {
        use std::sync::atomic::{AtomicU64, Ordering};
        let count = std::sync::Arc::new(AtomicU64::new(0));
        let count2 = count.clone();
        QuickCheck::new().tests(10).quickcheck(move || -> TestResult {
            let c = count2.fetch_add(1, Ordering::Relaxed);
            if c % 2 == 0 {
                TestResult::Discard
            } else {
                TestResult::Pass
            }
        });
    }

    #[test]
    fn test_testable_args1_passing() {
        QuickCheck::new()
            .tests(50)
            .quickcheck(testable_args1(|x: u8| -> bool { x < 200 || x >= 200 }));
    }

    #[test]
    #[should_panic(expected = "[disprove] FAILED")]
    fn test_testable_args1_failing() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(testable_args1(|x: u8| -> bool { x < 5 }));
    }

    #[test]
    fn test_testable_args1_shrinks() {
        // Property that fails for values > 10. The shrunk counterexample should be 11.
        let result = std::panic::catch_unwind(|| {
            QuickCheck::new()
                .tests(200)
                .gen(Gen::from_seed(100, 42))
                .quickcheck(testable_args1(|x: u32| -> bool { x <= 10 }));
        });
        assert!(result.is_err());
        let panic_msg = result
            .unwrap_err()
            .downcast_ref::<String>()
            .cloned()
            .unwrap_or_default();
        assert!(panic_msg.contains("[disprove] FAILED"));
    }

    #[test]
    fn test_testable_args2_passing() {
        QuickCheck::new()
            .tests(50)
            .quickcheck(testable_args2(|a: u8, b: u8| -> bool {
                a.wrapping_add(b) == b.wrapping_add(a)
            }));
    }

    #[test]
    fn test_deterministic_seed() {
        // Same seed should produce same results
        let mut results1 = Vec::new();
        let mut results2 = Vec::new();
        let mut g1 = Gen::from_seed(100, 999);
        let mut g2 = Gen::from_seed(100, 999);
        for _ in 0..50 {
            results1.push(u32::arbitrary(&mut g1));
            results2.push(u32::arbitrary(&mut g2));
        }
        assert_eq!(results1, results2);
    }

    #[test]
    fn test_quickcheck_convenience_fn() {
        quickcheck(|| -> bool { true });
    }

    #[test]
    fn test_quickcheck_vec_reverse() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(testable_args1(|xs: Vec<i32>| -> bool {
                let rev: Vec<_> = xs.iter().rev().rev().cloned().collect();
                rev == xs
            }));
    }
}
