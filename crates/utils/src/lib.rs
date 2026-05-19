use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into();
        if !Self::is_valid(&value) {
            return Err(NameError);
        }
        Ok(Self(value))
    }

    pub fn from_static(value: &'static str) -> Self {
        Self::new(value).expect("static name must be valid")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_valid(value: &str) -> bool {
        let mut chars = value.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        if !first.is_ascii_alphabetic() {
            return false;
        }
        chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    }
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Name {
    type Err = NameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameError;

impl fmt::Display for NameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "name must start with a letter and contain only letters, numbers, '_' or '-'",
        )
    }
}

impl std::error::Error for NameError {}
