use crate::objects::ocel::Ocel;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub(crate) fn import_json_ocel(file_path: &str) -> Result<Ocel, Box<dyn Error>> {
    let mut s = String::new();
    File::open(file_path).unwrap().read_to_string(&mut s).unwrap();
    Ok(serde_json::from_str(&s).unwrap())
}
