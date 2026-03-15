use std::fmt;

/// Error returned when a property test fails.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestError {
    /// Description of the failure.
    pub message: String,
    /// Number of successful tests before the failure.
    pub passed: u64,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "property test failed after {} passed tests: {}",
            self.passed, self.message
        )
    }
}

impl std::error::Error for TestError {}
