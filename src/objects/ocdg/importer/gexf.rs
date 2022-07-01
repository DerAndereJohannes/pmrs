use std::{error::Error, fs::File, io::Read};
use quick_xml::de::from_str;

use crate::objects::ocdg::variants::gexf::Gexf;


pub fn import_gexf_ocdg(file_path: &str) -> Result<Gexf, Box<dyn Error>> {
   let mut s = String::new();
   File::open(file_path).unwrap().read_to_string(&mut s).unwrap();
   let g: Gexf = from_str(&s)?;

   Ok(g)
}
