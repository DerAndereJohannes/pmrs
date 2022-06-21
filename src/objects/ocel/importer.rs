pub(crate) mod variants;
use crate::objects::ocel::Ocel;
use crate::objects::ocel::importer::variants::jsonocel::import_json_ocel;
use std::error::Error;

pub fn import_ocel(file_path: &str) -> Result<Ocel, Box<dyn Error>> {
    return import_json_ocel(file_path);
}
