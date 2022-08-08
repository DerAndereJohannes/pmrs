pub(crate) mod variants;
use std::error::Error;

use crate::objects::ocel::Ocel;

use self::variants::jsonocel::{export_json_ocel, export_json_ocel_pretty, generate_ocel_serde};

use super::OcelSerde;

pub fn export_ocel(log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_json_ocel(log, file_path);
}

pub fn export_ocel_pretty(log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_json_ocel_pretty(log, file_path);
}

pub fn generate_ocel_external_repr(log: &Ocel) -> OcelSerde {
    return generate_ocel_serde(log); 
}
