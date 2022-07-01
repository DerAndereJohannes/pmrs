pub(crate) mod variants;
use std::error::Error;

use crate::objects::ocel::Ocel;

use self::variants::jsonocel::export_json_ocel;

pub fn export_ocel(log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_json_ocel(log, file_path);
}
