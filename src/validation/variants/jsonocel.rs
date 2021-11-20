use jsonschema::JSONSchema;
use std::error::Error;
use crate::importer::import_ocel;

pub(crate) fn validate_json(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log = import_ocel(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json")).expect("JSON schema has been moved?");
    let compiled = JSONSchema::compile(&schema).expect("Schema is not valid.");

    Ok(compiled.is_valid(&log))
}

pub(crate) fn validate_json_verbose(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log = import_ocel(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json"))?;
    let compiled = JSONSchema::compile(&schema).expect("A valid Schema");

    let result = compiled.validate(&log);

    if let Err(errors) = result {
        for (i, error) in errors.enumerate() {
            println!("Error {}: {} at {}", i+1, error, error.instance_path);
        }
        Ok(false)
    } else {
        Ok(true)
    }
}
