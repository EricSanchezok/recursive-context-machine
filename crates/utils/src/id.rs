use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize};

static NEXT_COUNTER: AtomicU64 = AtomicU64::new(1);

fn unique_suffix(prefix: &str) -> String {
    let counter = NEXT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    prefix.hash(&mut hasher);
    nanos.hash(&mut hasher);
    counter.hash(&mut hasher);
    std::process::id().hash(&mut hasher);

    format!("{counter:08x}{:016x}", hasher.finish())
}

macro_rules! define_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
        pub struct $name(String);

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self(format!("{}{}", $prefix, unique_suffix($prefix)))
            }

            pub fn from_raw(value: String) -> Result<Self, IdError> {
                if value.starts_with($prefix) && value.len() > $prefix.len() {
                    Ok(Self(value))
                } else {
                    Err(IdError {
                        expected_prefix: $prefix,
                    })
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<DeserializerType>(
                deserializer: DeserializerType,
            ) -> Result<Self, DeserializerType::Error>
            where
                DeserializerType: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::from_raw(value).map_err(serde::de::Error::custom)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdError {
    expected_prefix: &'static str,
}

impl fmt::Display for IdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "id must start with '{}'", self.expected_prefix)
    }
}

impl std::error::Error for IdError {}

define_id!(GraphId, "graph_");
define_id!(AcceleratorId, "accel_");
define_id!(MachineId, "rcm_");
define_id!(FluxId, "flux_");
define_id!(ConditionId, "cond_");
define_id!(EnvironmentId, "env_");
define_id!(ResourcesId, "res_");
