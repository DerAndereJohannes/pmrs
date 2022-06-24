use crate::objects::ocel::Ocel;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

pub(crate) fn import_json_ocel(file_path: &str) -> Result<Ocel, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let log: Ocel = serde_json::from_reader(reader)?;

    Ok(log)
}
