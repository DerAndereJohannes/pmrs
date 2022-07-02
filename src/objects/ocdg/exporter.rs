pub(crate) mod variants;

use std::error::Error;

use crate::objects::{ocdg::variants::gexf::Gexf, ocel::Ocel};

use self::variants::gexf::{export_gexf_ocdg, export_gexf_ocdg_pretty};

use super::Ocdg;


pub fn export_ocdg(g: &Ocdg, log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_gexf_ocdg(g, log, file_path);
}

pub fn export_ocdg_pretty(g: &Gexf, file_path: &str) -> Result<bool, Box<dyn Error>> {
    return export_gexf_ocdg_pretty(g, file_path);
}
