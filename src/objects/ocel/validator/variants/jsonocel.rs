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

pub(crate) fn validate_json_verbose(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let log = fs::read_to_string(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json"))?;
    let compiled = JSONSchema::compile(&schema).expect("What have you done with the existing json schema?");
    
    let json_log: Value = serde_json::from_str(&log.as_str())?;
    let result = compiled.validate(&json_log);
    let mut extracted_errors: Vec<(String, String)> = vec![];

    if let Err(errors) = result {
        extracted_errors.extend(errors.into_iter()
                                      .map(|ierr| (ierr.to_string(), ierr.instance_path.to_string())));
    }

    Ok(extracted_errors)
}
