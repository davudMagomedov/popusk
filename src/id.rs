use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IDError {
    #[error("couldn't parse ID from string: '{s}'")]
    FromStr { s: String },
}

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
    type Err = IDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map(|integer_res| ID(integer_res))
            .map_err(|_| IDError::FromStr { s: s.to_owned() })
    }
}
