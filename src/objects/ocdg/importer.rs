pub(crate) mod gexf;
use std::error::Error;

use self::gexf::import_gexf_ocdg;

use super::variants::gexf::Gexf;

pub fn import_ocdg(file_path: &str) -> Result<Gexf, Box<dyn Error>> {
    return import_gexf_ocdg(file_path);
}
