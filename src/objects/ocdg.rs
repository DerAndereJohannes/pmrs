use std::panic;
use std::{collections::{HashMap, hash_map::Entry}, vec};
use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};
use serde_json::Value;

use super::ocel::Ocel;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Relations {
    INTERACTS,
    COLIFE,
    COBIRTH,
    CODEATH,
    DESCENDANTS,
    INHERITANCE,
    CONSUMES,
    SPLIT,
    MERGE,
    MINION,
    PEELER,
    PARTOF,
    ENGAGES
}

impl Relations {
    fn collect<'a>(&self, _log: &Ocel, ocdg: &Ocdg, eid: &'a str, oid1: &'a str, oid2: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut edges: Vec<(&str, &str)> = vec![];
        
        match self {
            Relations::INTERACTS => {
                edges.push((oid1, oid2));
            }
            Relations::COLIFE => todo!(),
            Relations::COBIRTH => todo!(),
            Relations::CODEATH => todo!(),
            Relations::DESCENDANTS => {
                let src_oe = ocdg.get_oe(&oid1).unwrap();
                let tar_oe = ocdg.get_oe(&oid2).unwrap();
                if &eid == tar_oe.first().unwrap() && &eid != src_oe.first().unwrap() {
                    edges.push((oid1, oid2))
                }
            },
            Relations::INHERITANCE => todo!(),
            Relations::CONSUMES => todo!(),
            Relations::SPLIT => todo!(),
            Relations::MERGE => todo!(),
            Relations::MINION => todo!(),
            Relations::PEELER => todo!(),
            Relations::PARTOF => todo!(),
            Relations::ENGAGES => todo!(),
        }
        

        edges
    }
    
}

#[derive(Debug)]
pub enum NodeValue<'a> {
    ATTR(&'a Value),
    ATTROWN(Value),
    INDEX(NodeIndex),
    OE(Vec<&'a str>)
}

pub struct Ocdg<'a> {
    pub net: DiGraph<&'a str, &'a str>,
    pub edge_attributes: HashMap<&'a str, HashMap<&'a str, NodeValue<'a>>>,
    pub node_attributes: HashMap<&'a str, HashMap<&'a str, NodeValue<'a>>>,
    pub inodes: HashMap<&'a str, NodeIndex>,
    pub iedges: HashMap<(&'a str, &'a str), EdgeIndex>,
    pub irels: HashMap<(&'a str, &'a str), HashMap<&'a Relations, Vec<&'a str>>>
}

impl Default for Ocdg<'_> {
    fn default() -> Self {
        Self{
            net: DiGraph::<&str, &str>::new(),
            edge_attributes: HashMap::new(),
            node_attributes: HashMap::new(),
            inodes: HashMap::new(),
            iedges: HashMap::new(),
            irels: HashMap::new()
        }
    }
}


impl<'a> Ocdg<'a> {

    fn init_object_key(&mut self, oid: &'a str) {
        self.node_attributes.insert(&oid, HashMap::<&str, NodeValue>::new());
    }

    fn add_object_value(&mut self, oid: &'a str, key: &'a str, value: NodeValue<'a>) {
        let oid_root = self.node_attributes.get_mut(&oid).unwrap();

        oid_root.entry(key)
                .or_insert(value);


    }

    fn add_eid_to_oe(&mut self, oid: &'a str, eid: &'a String) {
        let oe_entry = self.node_attributes.entry(&oid).or_default().entry("object_events");
        match oe_entry {
            Entry::Vacant(_e) => { panic!("The object seems to have skipped initialization!"); },
            Entry::Occupied(mut e) => { 
                if let NodeValue::OE(oe) = e.get_mut() {
                    oe.push(eid.as_str());
                } 
            
            }

        }
    }

    fn get_node_value(&self, node: &'a str, attr: &'a str) -> Option<&NodeValue> {
        Some(&self.node_attributes[&node][&attr])

    }

    fn get_oe(&self, node: &'a str) -> Option<&Vec<&str>> {
        if let NodeValue::OE(oe) = self.get_node_value(node, "object_events").unwrap() {
            Some(oe)
        } else {
            None
        }
    }

    fn apply_new_edges(&mut self, new_edges: Vec<(&'a str, &'a str)>, eid: &'a str, rel: &'a Relations) {
         for edge in &new_edges {
            let new_edge = self.net.add_edge(self.inodes[edge.0], self.inodes[edge.1], eid);
            self.iedges.entry(*edge).or_insert(new_edge);

            
            match self.irels.entry(*edge).or_default().entry(rel) {
                Entry::Vacant(e) => {
                    e.insert(vec![eid]);
                },
                Entry::Occupied(mut e) => {
                    e.get_mut().push(eid);
                    
                }
            }
         }
    }

}

pub fn str_to_value(new_string: &String) -> NodeValue {
    NodeValue::ATTROWN(Value::String(new_string.clone()))
    
}

pub fn generate_ocdg<'a>(log: &'a Ocel, relations: &'a Vec<Relations>) -> Ocdg<'a> {
    let mut ocdg: Ocdg = Ocdg::default();
    
    for (eid, data) in &log.events {
        for oid in &data.omap {
            let oid_str: &str = &oid.as_str();
            if !ocdg.node_attributes.contains_key(oid_str) {
                let new_node = ocdg.net.add_node(&oid_str);
                ocdg.init_object_key(&oid_str);
                ocdg.inodes.entry(&oid_str).or_insert(new_node);
                ocdg.add_object_value(&oid_str, "type", str_to_value(&log.objects[&oid_str.to_string()].obj_type)); 
                ocdg.add_object_value(&oid_str, "object_events", NodeValue::OE(vec![])); 

            }
            
            ocdg.add_eid_to_oe(&oid_str, &eid);
        }
    }

    for (eid, data) in &log.events {
        for oid1 in &data.omap {
            for oid2 in &data.omap {
                if oid1 != oid2 {
                    for rel in relations {
                        let new_edges = rel.collect(&log, &ocdg, &eid, &oid1, &oid2);
                        if new_edges.len() > 0 {
                            ocdg.apply_new_edges(new_edges, &eid, rel);

                        }
                    }
                }

            }    
        } 
    }

    ocdg
}
