use crate::blueprint::wrapping_type;
use crate::is_default;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::num::NonZeroU64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub types: BTreeMap<String, Type1>,
    pub upstream: Upstream,
    pub server: Server,
}

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct Server {
    #[serde(default, skip_serializing_if = "is_default")]
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 8000,
        }
    }
}

#[derive(Serialize, Deserialize,Clone, Debug, Default)]
pub struct Upstream {
    #[serde(rename = "baseURL", default, skip_serializing_if = "is_default")]
    base_url: Option<String>,
}

// TODO: rename
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Type1 {
    pub fields: BTreeMap<String, Field>,
    pub cache: Option<Cache>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Cache {
    pub max_age: NonZeroU64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Field {
    pub ty_of: wrapping_type::Type,
    pub resolver: Option<Resolver>,
    pub args: BTreeMap<String, Arg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Arg {
    pub type_of: wrapping_type::Type,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Http {
    pub path: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Resolver {
    Http(Http),
}
