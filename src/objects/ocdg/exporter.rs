pub(crate) mod variants;

use std::error::Error;

use self::variants::gexf::{export_gexf_ocdg, ocdg_to_xml};

use super::Ocdg;


pub fn generate_ocdg_string(g: &Ocdg) -> Result<String, Box<dyn Error>> {
    return ocdg_to_xml(g);
}

pub fn export_ocdg(g: &Ocdg, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_gexf_ocdg(g, file_path);
}
