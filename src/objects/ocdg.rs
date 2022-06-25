use std::{collections::hash_map::Entry, vec};
use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};
use serde_json::Value;
use ahash::{AHashMap, AHashSet};

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
    ENGAGES
}

impl Relations {
    fn collect<'a>(&self, log: &Ocel, ocdg: &Ocdg, eid: &'a str, oid1: &'a str, oid2: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut edges: Vec<(&str, &str)> = vec![];
        let src_oe = &ocdg.node_attributes.get(&oid1).unwrap().object_events;
        let tar_oe = &ocdg.node_attributes.get(&oid2).unwrap().object_events;
        let src_type = &ocdg.node_attributes.get(&oid1).unwrap().node_type;
        let tar_type = &ocdg.node_attributes.get(&oid2).unwrap().node_type;
        
        match self {
            Relations::INTERACTS => {
                edges.push((oid1, oid2));
            }
            Relations::COLIFE => { // one time
                if src_oe == tar_oe {
                    edges.push((oid1, oid2));
                }
            },
            Relations::COBIRTH => { // one time
                if src_oe.first().unwrap() == tar_oe.first().unwrap() {
                    edges.push((oid1, oid2));
                }
            },
            Relations::CODEATH => { // one time
                if !ocdg.irels.get(&(oid1, oid2)).unwrap().contains_key(&Relations::CODEATH) &&
                   src_oe.last().unwrap() == tar_oe.last().unwrap() {
                    edges.push((oid1, oid2));
                    edges.push((oid2, oid1));
                }
            },
            Relations::DESCENDANTS => { // one time
                if &eid == tar_oe.first().unwrap() && &eid != src_oe.first().unwrap() {
                    edges.push((oid1, oid2));
                }
            },
            Relations::INHERITANCE => {
                if src_type == tar_type &&
                   &eid == src_oe.last().unwrap() && 
                   &eid == tar_oe.first().unwrap() {
                    edges.push((oid1, oid2));
                }
            },
            Relations::CONSUMES => {
                if src_type != tar_type &&
                   &eid == src_oe.last().unwrap() && 
                   &eid == tar_oe.first().unwrap() {
                    edges.push((oid1, oid2));
                }

            },
            Relations::SPLIT => {
                if src_type == tar_type && 
                   &eid == src_oe.last().unwrap() &&
                   &eid == tar_oe.first().unwrap() {
                    let omap = &log.events[&eid.to_string()].omap;
                    let children: Vec<_> = omap.iter()
                                               .filter(|o| src_type == &log.objects[&o.to_string()].obj_type.as_str() && 
                                                            o != &oid1 &&
                                                           // &eid == ocdg.get_oe(o).unwrap().first().unwrap())
                                                           &eid == ocdg.node_attributes.get(o as &str).unwrap().object_events.first().unwrap())
                                               .collect();

                    if children.len() > 1 {
                        edges.push((oid1, oid2))
                    } 
                }
            },
            Relations::MERGE => {
                if src_type == tar_type && 
                   &eid == src_oe.last().unwrap() &&
                   &eid != tar_oe.last().unwrap() {
                       edges.push((oid1, oid2));
                }

            },
            Relations::MINION => {
                   if src_oe.len() > tar_oe.len() {
                       let src_oe_set: AHashSet<&&str> = AHashSet::from_iter(src_oe.iter());
                       let tar_oe_set: AHashSet<&&str> = AHashSet::from_iter(tar_oe.iter());

                       if tar_oe_set.is_subset(&src_oe_set) {
                            edges.push((oid1, oid2));
                       }
                   }
            },
            Relations::PEELER => {
                    let shorter_oe = if src_oe.len() > tar_oe.len() {tar_oe} else {src_oe};

                    for event in shorter_oe.iter() {
                        let omap = &log.events.get(&event.to_string()).unwrap().omap;    
                        if omap.len() > 2 && omap.contains(oid1) && omap.contains(oid2){ 
                            return edges; 
                        }
                    }
                    edges.push((oid1, oid2));
                    edges.push((oid2, oid1));

            },
            Relations::ENGAGES => {
                    let src_oe_set: AHashSet<&&str> = AHashSet::from_iter(src_oe.iter());
                    let tar_oe_set: AHashSet<&&str> = AHashSet::from_iter(tar_oe.iter());
                    if !tar_oe_set.contains(src_oe.first().unwrap()) &&
                       !tar_oe_set.contains(src_oe.last().unwrap()) &&
                       !src_oe_set.contains(tar_oe.first().unwrap()) &&
                       !src_oe_set.contains(tar_oe.last().unwrap()) {
                            edges.push((oid1, oid2));
                            edges.push((oid2, oid1));
                       }

            },
        }
        

        edges
    }
    
}


#[derive(Debug)]
pub enum NodeValue<'a> {
    ATTR(&'a Value),
    OE(Vec<&'a str>),
    ATTRSTR(&'a str)
}

#[derive(Debug, Default)]
pub struct NodeInfo<'a> {
    pub node_type: &'a str,
    pub object_events: Vec<&'a str>
}

pub struct Ocdg<'a> {
    pub net: DiGraph<&'a str, &'a str>,
    pub edge_attributes: AHashMap<&'a str, NodeInfo<'a>>,
    pub node_attributes: AHashMap<&'a str, NodeInfo<'a>>,
    pub inodes: AHashMap<&'a str, NodeIndex>,
    pub iedges: AHashMap<(&'a str, &'a str), EdgeIndex>,
    pub irels: AHashMap<(&'a str, &'a str), AHashMap<&'a Relations, Vec<&'a str>>>
}

impl Default for Ocdg<'_> {
    fn default() -> Self {
        Self{
            net: DiGraph::<&str, &str>::new(),
            edge_attributes: AHashMap::new(),
            node_attributes: AHashMap::new(),
            inodes: AHashMap::new(),
            iedges: AHashMap::new(),
            irels: AHashMap::new()
        }
    }
}


impl<'a> Ocdg<'a> {

    fn init_object_key(&mut self, oid: &'a str) {
        self.node_attributes.insert(&oid, NodeInfo::default());
    }


    fn add_eid_to_oe(&mut self, oid: &'a str, eid: &'a String) {
        let oe_entry: &mut Vec<&str> = &mut self.node_attributes.entry(&oid).or_default().object_events;
        oe_entry.push(eid.as_str());

    }

    fn apply_new_edges(&mut self, new_edges: Vec<(&'a str, &'a str)>, eid: &'a str, rel: &'a Relations) {
         for edge in &new_edges {
            self.iedges.entry(*edge).or_insert_with(|| self.net.add_edge(self.inodes[edge.0], self.inodes[edge.1], ""));
            
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

pub fn generate_ocdg<'a>(log: &'a Ocel, relations: &'a Vec<Relations>) -> Ocdg<'a> {
    let mut ocdg: Ocdg = Ocdg::default();
    
    for (eid, data) in &log.events {
        for oid in &data.omap {
            let oid_str: &str = &oid.as_str();
            if !ocdg.node_attributes.contains_key(oid_str) {
                let new_node = ocdg.net.add_node(&oid_str);
                ocdg.init_object_key(&oid_str);
                ocdg.inodes.entry(&oid_str).or_insert(new_node);
                let curr_obj = &log.objects[&oid_str.to_string()];
                ocdg.node_attributes.entry(&oid_str).or_default().node_type = &curr_obj.obj_type;
                ocdg.node_attributes.entry(&oid_str).or_default().object_events = vec![];

            }
            
            ocdg.add_eid_to_oe(&oid_str, &eid);
        }
    }

    for (eid, data) in &log.events {
        for oid1 in &data.omap {
            for oid2 in &data.omap {
                if oid1 != oid2 {
                    for rel in relations {
                        if rel == &Relations::INTERACTS || !ocdg.irels.get(&(&oid1.as_str(), &oid2.as_str())).unwrap().contains_key(rel) {
                            let new_edges = rel.collect(&log, &ocdg, &eid, &oid1, &oid2);
                            if new_edges.len() > 0 {
                                ocdg.apply_new_edges(new_edges, &eid, rel);
                            }
                        }
                    }
                }

            }    
        } 
    }

    ocdg
}
