pub(crate) mod variants;
use std::error::Error;

use crate::objects::ocel::Ocel;

use self::variants::gexf::{import_gexf_ocdg, import_gexf_ocdg_link_ocel};

use super::Ocdg;

pub fn import_ocdg(file_path: &str) -> Result<Ocdg, Box<dyn Error>> {
    return import_gexf_ocdg(file_path);
}

pub fn import_ocdg_link_ocel(file_path: &str, log: &Ocel) -> Result<Ocdg, Box<dyn Error>> {
    return import_gexf_ocdg_link_ocel(file_path, log);
}
