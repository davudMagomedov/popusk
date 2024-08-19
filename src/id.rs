use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::{Context, Error as AnyhowError};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ID(u64);

impl ID {
    pub fn new(v: u64) -> Self {
        ID(v)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ID {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ID(s
            .parse::<u64>()
            .context(format!("couldn't parse '{}' as an ID", s))?))
    }
}
