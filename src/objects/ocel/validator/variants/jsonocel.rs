use jsonschema::JSONSchema;
use serde_json::to_value;
use std::error::Error;
use crate::objects::ocel::importer::import_ocel;

pub(crate) fn validate_json(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log = import_ocel(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json")).expect("JSON schema has been moved?");
    let compiled = JSONSchema::compile(&schema).expect("Schema is not valid.");

    Ok(compiled.is_valid(&to_value(&log).unwrap()))
}

pub(crate) fn validate_json_verbose(file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log = import_ocel(file_path)?;
    let schema =  serde_json::from_str(include_str!("schema.json"))?;
    let compiled = JSONSchema::compile(&schema).expect("A valid Schema");
    
    let json_log = to_value(&log).unwrap();
    println!("{}", &json_log.to_string());
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
