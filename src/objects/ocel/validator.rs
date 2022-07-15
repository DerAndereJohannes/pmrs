pub(crate) mod variants;
use std::error::Error;
use crate::objects::ocel::validator::variants::jsonocel::{validate_json, validate_json_verbose};

pub fn validate_ocel(file_path: &str) -> Result<bool, Box<dyn Error>>{
    return validate_json(file_path);
}

pub fn validate_ocel_verbose(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>>{
    return validate_json_verbose(file_path);
}
