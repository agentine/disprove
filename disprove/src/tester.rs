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

/// Trait for types that can be tested as properties.
pub trait Testable {
    /// Run this property with the given generator.
    fn result(&self, g: &mut Gen) -> TestResult;
}

/// Configurable property-based test runner.
pub struct QuickCheck {
    tests: u64,
    max_tests: u64,
    gen: Gen,
}

impl QuickCheck {
    /// Create a new QuickCheck runner with default settings.
    pub fn new() -> Self {
        QuickCheck {
            tests: 100,
            max_tests: 10_000,
            gen: Gen::new(100),
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

    /// Set the generator to use.
    pub fn gen(mut self, gen: Gen) -> Self {
        self.gen = gen;
        self
    }

    /// Run the property test. Panics on failure.
    pub fn quickcheck<T: Testable>(&mut self, _testable: T) {
        // Placeholder — full implementation in Phase 4
        let _ = (self.tests, self.max_tests, &mut self.gen);
    }
}

impl Default for QuickCheck {
    fn default() -> Self {
        Self::new()
    }
}

/// Run a property test with default settings.
pub fn quickcheck<T: Testable>(_testable: T) {
    // Placeholder — full implementation in Phase 4
}
