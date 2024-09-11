// Just a reader.. nothing special here

use crate::config::Config;
use std::path::Path;

pub struct ConfigReader {

}

impl ConfigReader {
    pub fn init() -> Self {
        Self {

        }
    }
    pub fn read<T: AsRef<Path>>(path: T) -> anyhow::Result<Config> {
        let sdl = std::fs::read_to_string(path)?;
        let doc = async_graphql::parser::parse_schema(sdl)?;
        Ok(crate::from_doc::from_doc(doc)?)
    }
}