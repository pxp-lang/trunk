use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonFile {
    require: HashMap<String, String>,
}

impl JsonFile {
    pub fn get_requires(&self) -> &HashMap<String, String> {
        &self.require
    }

    pub fn get_require(&self, name: &str) -> Option<&String> {
        self.require.get(name)
    }
}

pub fn get_json_file() -> std::io::Result<JsonFile> {
    let file = std::fs::read("composer.json")?;
    let json_file: JsonFile = serde_json::from_slice(&file)?;
    Ok(json_file)
}