pub(crate) mod variants;

use std::error::Error;

use crate::objects::ocel::Ocel;

use self::variants::gexf::{export_gexf_ocdg, export_gexf_ocdg_string};

use super::Ocdg;


pub fn export_ocdg_string(g: &Ocdg, log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_gexf_ocdg_string(g, log, file_path);
}

pub fn export_ocdg(g: &Ocdg, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_gexf_ocdg(g, file_path);
}
