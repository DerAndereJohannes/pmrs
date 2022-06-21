use std::collections::HashMap;
use petgraph::graph::DiGraph;
use serde_json::Value;

use super::ocel::Ocel;

pub struct Ocdg {
    pub net: DiGraph<String, i32>,
    pub edge_attributes: HashMap<String, HashMap<String, Value>>,
    pub node_attributes: HashMap<String, HashMap<String, Value>>
}

impl Default for Ocdg {
    fn default() -> Self {
        Self{
            net: DiGraph::<String, i32>::new(),
            edge_attributes: HashMap::<String, HashMap::<String, Value>>::new(),
            node_attributes: HashMap::<String, HashMap::<String, Value>>::new()}
    }
}


pub fn generate_ocdg(log: &Ocel, relations: Vec<String>) -> Ocdg {
    let mut ocdg: Ocdg = Ocdg::default();
    ocdg
}
