use serde_derive::{Deserialize, Serialize};

/// INVARIANTS:
/// - `passed <= ceiling`
/// - `ceiling >= 1`
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Progress {
    passed: usize,
    ceiling: usize,
}

impl Progress {
    pub fn new(ceiling: usize) -> Self {
        if ceiling == 0 {
            panic!("Trying to make `Progress {{ .., ceiling: 0 }}`");
        }

        Progress { passed: 0, ceiling }
    }

    pub fn with_passed(passed: usize, ceiling: usize) -> Self {
        if ceiling == 0 || passed > ceiling {
            panic!("Trying to make `Progress {{ .. }}` with broke invariants");
        }

        Progress { passed, ceiling }
    }

    pub fn passed(&self) -> usize {
        self.passed
    }

    pub fn ceiling(&self) -> usize {
        self.ceiling
    }

    /// Returns error if new passed value is invalid (if it brakes the invariants)
    pub fn set_passed(&mut self, new_passed: usize) {
        if new_passed > self.ceiling {
            panic!("Trying to change `Progress` braking invariants");
        }

        self.passed = new_passed;
    }

    pub fn forward_by(&mut self, by: usize) {
        self.passed = (self.passed + by).min(self.ceiling);
    }

    pub fn backward_by(&mut self, by: usize) {
        self.passed = self.passed.checked_sub(by).unwrap_or(0);
    }
}
