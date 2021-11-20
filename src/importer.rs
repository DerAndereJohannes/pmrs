mod variants;
use serde_json::Value as JsonValue;
use crate::importer::variants::jsonocel::import_json_ocel;
use std::error::Error;

pub fn import_ocel(file_path: &str) -> Result<JsonValue, Box<dyn Error>> {
    return import_json_ocel(file_path);
}
