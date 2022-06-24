use jsonschema::JSONSchema;
use serde_json::Value;
use std::{error::Error, fs};

pub(crate) fn validate_json(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log = fs::read_to_string(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json")).expect("JSON schema has been moved?");
    let compiled = JSONSchema::compile(&schema).expect("Schema is not valid.");
    let json_log: Value = serde_json::from_str(&log.as_str()).unwrap();

    Ok(compiled.is_valid(&json_log))
}

pub(crate) fn validate_json_verbose(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log = fs::read_to_string(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json"))?;
    let compiled = JSONSchema::compile(&schema).expect("A valid Schema");
    
    let json_log: Value = serde_json::from_str(&log.as_str()).unwrap();
    let result = compiled.validate(&json_log);

    if let Err(errors) = result {
        for (i, error) in errors.enumerate() {
            println!("Error {}: {} at {}", i+1, error, error.instance_path);
        }
        Ok(false)
    } else {
        Ok(true)
    }
}
