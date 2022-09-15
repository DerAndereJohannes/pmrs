pub(crate) mod variants;
pub mod importer;
pub mod exporter;
pub mod decomposition;
pub(crate) mod generation;

use std::{collections::hash_map::Entry, vec, fmt, error::Error};
use ahash::AHashSet;
use bimap::BiMap;
use petgraph::{graph::{NodeIndex, EdgeIndex}, stable_graph::StableDiGraph};
use nohash_hasher::{IntSet, IntMap};
use rayon::prelude::*;
use num_enum::{TryFromPrimitive, IntoPrimitive};
use strum::{EnumIter, EnumString};

use super::ocel::Ocel;


#[derive(Debug, Clone, PartialEq)]
pub struct RelationError;

impl fmt::Display for RelationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Relation input in vector")
    }
}

impl Error for RelationError {}

pub trait OcdgRelations {
    fn is_timeconscious(&self) -> bool;
    fn is_directed(&self) -> bool;
    fn is_multiproof(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, TryFromPrimitive, IntoPrimitive, EnumIter, EnumString)]
#[repr(u8)]
pub enum Relations {
    INTERACTS = 0,
    COLIFE = 1,
    COBIRTH = 2,
    CODEATH = 3,
    DESCENDANTS = 4,
    INHERITANCE = 5,
    CONSUMES = 6,
    SPLIT = 7,
    MERGE = 8,
    MINION = 9,
    PEELER = 10,
    ENGAGES = 11,
    ASCENDANTS = 12
}

impl OcdgRelations for Relations {
    fn is_timeconscious(&self) -> bool {
        match self {
            Relations::DESCENDANTS | Relations::INHERITANCE | Relations::SPLIT | Relations::CONSUMES => {true},
            _ => {false}
        }
    }

    fn is_directed(&self) -> bool {
        match self {
            Relations::DESCENDANTS | Relations::INHERITANCE | Relations::SPLIT | Relations::CONSUMES | Relations::MINION | Relations::ASCENDANTS => {true},
            _ => {false}
        }
    }

    fn is_multiproof(&self) -> bool {
        match self {
            Relations::COLIFE | Relations::PEELER | Relations::ENGAGES => {true},
            _ => {false}
        }
    }
}

impl fmt::Display for Relations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Relations {
    fn relation_type(&self) -> u8 {
        match self {
            Relations::SPLIT => 1,
            _ => {2}
            
        }
    }

    pub fn relation_index(&self) -> u8 {
        match self {
            Relations::INTERACTS => 0,
            Relations::COLIFE => 1,
            Relations::COBIRTH => 2,
            Relations::CODEATH => 3,
            Relations::DESCENDANTS => 4,
            Relations::INHERITANCE => 5,
            Relations::CONSUMES => 6,
            Relations::SPLIT => 7,
            Relations::MERGE => 8,
            Relations::MINION => 9,
            Relations::PEELER => 10,
            Relations:: ENGAGES => 11,
            Relations::ASCENDANTS => 12
        }
    }

    
    fn execute_whole(&self, log: &Ocel, ocdg: &Ocdg, neighs: &IntMap<usize, IntSet<usize>>, oid1: usize) -> Vec<(usize, usize, EventAdd, Relations)> {
        let mut to_add: Vec<(usize, usize, EventAdd, Relations)> = Vec::new();
        let src_oe = &log.objects.get(&oid1).unwrap().events;
        let src_type = &ocdg.node_attributes.get(&oid1).unwrap().node_type;
            match self {
                Relations::SPLIT => {
                    let mut conforming_oid: IntSet<usize> = IntSet::default();
                    let src_e = src_oe.last().unwrap();
                    for oid2 in neighs.get(&oid1).unwrap() {
                        let neigh_oe = &log.objects.get(&oid2).unwrap().events;
                        let neigh_type = &ocdg.node_attributes.get(&oid2).unwrap().node_type;
                        if src_type == neigh_type && src_e == neigh_oe.first().unwrap() {
                            conforming_oid.insert(*oid2);
                        }
                        
                    }
                    if conforming_oid.len() > 1 {
                        for oid2 in &conforming_oid {
                            to_add.push((oid1, *oid2, EventAdd::SINGLE(*src_e), Relations::SPLIT));

                        }
                    }
                },
                _ => {},
            }
            to_add
        }


    fn execute(&self, log: &Ocel, ocdg: &Ocdg, oid1: usize, oid2: usize) -> Vec<(usize, usize, EventAdd, Relations)> {
        let mut to_add: Vec<(usize, usize, EventAdd, Relations)> = Vec::new();
        let src_oe = &log.objects.get(&oid1).unwrap().events;
        let tar_oe = &log.objects.get(&oid2).unwrap().events;
        let src_type = &ocdg.node_attributes.get(&oid1).unwrap().node_type;
        let tar_type = &ocdg.node_attributes.get(&oid2).unwrap().node_type;
        
        match self {
            Relations::INTERACTS => {
                if oid1 < oid2 {
                    let e_set = intersection_count_sorted_vec(src_oe, tar_oe);
                    to_add.push((oid1, oid2, EventAdd::MULTI(e_set.to_owned()), Relations::INTERACTS));
                    to_add.push((oid2, oid1, EventAdd::MULTI(e_set), Relations::INTERACTS));
                }
            },
            Relations::DESCENDANTS => {
                if (src_oe[0] < tar_oe[0]) && src_oe.contains(&tar_oe[0]) {
                    to_add.push((oid1, oid2, EventAdd::SINGLE(tar_oe[0]), Relations::DESCENDANTS));
                }
            },
            Relations::ASCENDANTS => {
                if (src_oe[0] < tar_oe[0]) && src_oe.contains(&tar_oe[0]) {
                    to_add.push((oid2, oid1, EventAdd::SINGLE(tar_oe[0]), Relations::ASCENDANTS));
                }
            },
            Relations::COLIFE => { // one time
                if oid1 < oid2 && src_oe == tar_oe {
                    let e_set: IntSet<usize> = IntSet::from_iter(src_oe.to_owned());
                    to_add.push((oid1, oid2, EventAdd::MULTI(e_set.to_owned()), Relations::COLIFE));
                    to_add.push((oid2, oid1, EventAdd::MULTI(e_set), Relations::COLIFE));
                }
            },
            Relations::COBIRTH => { // one time
                if oid1 < oid2 {
                    let src_e = src_oe.first().unwrap();
                    if src_e == tar_oe.first().unwrap() {
                        to_add.push((oid1, oid2, EventAdd::SINGLE(*src_e), Relations::COBIRTH));
                        to_add.push((oid2, oid1, EventAdd::SINGLE(*src_e), Relations::COBIRTH));
                    }
                }
            },
            Relations::CODEATH => { // one time
                if oid1 < oid2 {
                    let src_e = src_oe.last().unwrap();
                    if src_e == tar_oe.last().unwrap() {
                        to_add.push((oid1, oid2, EventAdd::SINGLE(*src_e), Relations::CODEATH));
                        to_add.push((oid2, oid1, EventAdd::SINGLE(*src_e), Relations::CODEATH));
                    }
                }
            },
            Relations::INHERITANCE => {
                let src_e = src_oe.last().unwrap();
                if src_type == tar_type &&
                   src_e == tar_oe.first().unwrap() {
                    to_add.push((oid1, oid2, EventAdd::SINGLE(*src_e), Relations::INHERITANCE));
                }
            },
            Relations::CONSUMES => {
                let src_e = src_oe.last().unwrap();
                if src_type != tar_type &&
                   src_e == tar_oe.first().unwrap() {
                    to_add.push((oid1, oid2, EventAdd::SINGLE(*src_e), Relations::CONSUMES));
                }
            },
            Relations::MERGE => {
                let src_e = src_oe.last().unwrap();
                if src_type == tar_type && 
                   src_oe.last().unwrap() != tar_oe.last().unwrap() {
                    to_add.push((oid1, oid2, EventAdd::SINGLE(*src_e), Relations::MERGE));
                }
            },
            Relations::MINION => {
                   if src_oe.len() > tar_oe.len() {
                       let src_set: AHashSet<&usize> = AHashSet::from_iter(src_oe);
                       for ev in tar_oe {
                            if !src_set.contains(ev) {
                                return to_add;
                            }
                       }
                       to_add.push((oid1, oid2, EventAdd::MULTI(IntSet::<usize>::from_iter(tar_oe.iter().cloned())), Relations::MINION));
                   }
            },
            Relations::PEELER => {
                if oid1 < oid2  {
                    let shorter_oe = if src_oe.len() > tar_oe.len() {tar_oe} else {src_oe};
                    let mut shared_events: IntSet<usize> = IntSet::default();
                    let mut failed: bool = false;
                    for event in shorter_oe.iter() {
                        let omap = &log.events.get(&*event).unwrap().omap;    
                        if omap.len() > 2 && omap.contains(&oid1) && omap.contains(&oid2){ 
                            failed = true;
                            break; 
                        } else {
                            shared_events.insert(*event);
                        }
                    }
                    if !failed {
                        to_add.push((oid1, oid2, EventAdd::MULTI(shared_events.to_owned()), Relations::PEELER));
                        to_add.push((oid2, oid1, EventAdd::MULTI(shared_events), Relations::PEELER));
                    }
                }
            },
            Relations::ENGAGES => {
                if oid1 < oid2 {
                    let src_oe_set: IntSet<_> = IntSet::<usize>::from_iter(src_oe.clone());
                    let tar_oe_set: IntSet<_> = IntSet::<usize>::from_iter(tar_oe.clone());
                    if !tar_oe_set.contains(src_oe.first().unwrap()) &&
                       !tar_oe_set.contains(src_oe.last().unwrap()) &&
                       !src_oe_set.contains(tar_oe.first().unwrap()) &&
                       !src_oe_set.contains(tar_oe.last().unwrap()) {
                            let shared_events: IntSet<usize> = src_oe_set.intersection(&tar_oe_set).map(|i| *i).collect();
                            to_add.push((oid1, oid2, EventAdd::MULTI(shared_events.to_owned()), Relations::ENGAGES));
                            to_add.push((oid2, oid1, EventAdd::MULTI(shared_events), Relations::ENGAGES));
                       }
                }

            },
            _ => {}
        }
        to_add
    }
}


#[derive(Debug, Clone)]
pub enum EventAdd {
    SINGLE(usize),
    MULTI(IntSet<usize>)
}


#[derive(Debug, Default)]
pub struct NodeInfo {
    pub node_type: String,
    pub src_cut: IntSet<usize>,
    pub tar_cut: IntSet<usize>
}

#[derive(Debug, Default)]
pub struct Ocdg {
    pub net: StableDiGraph<usize, usize>,
    pub edge_attributes: IntMap<usize, NodeInfo>,
    pub node_attributes: IntMap<usize, NodeInfo>,
    pub object_map: BiMap<String, usize>,
    pub event_map: BiMap<String, usize>,
    pub inodes: IntMap<usize, NodeIndex>,
    pub iedges: IntMap<usize, IntMap<usize, EdgeIndex>>,
    pub irels: IntMap<usize, IntMap<usize,IntMap<u8, IntSet<usize>>>>
}

impl Ocdg {

    fn init_object_key(&mut self, oid: usize) {
        self.node_attributes.insert(oid, NodeInfo::default());
    }


    fn apply_new_edges(&mut self, edge: (usize, usize), eids: EventAdd, rel: Relations) {
            self.iedges.entry(edge.0).or_default().entry(edge.1).or_insert_with(|| self.net.add_edge(self.inodes[&edge.0], self.inodes[&edge.1], 0));
            match self.irels.entry(edge.0).or_default().entry(edge.1).or_default().entry(rel.relation_index().into()) {
                Entry::Vacant(e) => {
                    if let EventAdd::MULTI(multi) = eids {
                        e.insert(multi);
                    } else if let EventAdd::SINGLE(single) = eids {
                        let mut new_set: IntSet<usize> = IntSet::default();
                        new_set.insert(single);
                        e.insert(new_set);
                    }
                },
                Entry::Occupied(mut e) => {
                    if let EventAdd::MULTI(multi) = eids {
                        e.get_mut().extend(&multi);
                    } else if let EventAdd::SINGLE(single) = eids {
                        e.get_mut().insert(single);
                    }
                    
                }
            }  
    }


}

pub fn generate_ocdg(log: &Ocel, relations: &Vec<Relations>) -> Ocdg {
    let mut ocdg: Ocdg = Ocdg::default();
    let rel_inst: Vec<_> = relations.iter().filter(|r| r.relation_type() == 2).collect();
    let rel_whole: Vec<_> = relations.iter().filter(|r| r.relation_type() == 1).collect();
    let mut neighbours: IntMap<usize, IntSet<usize>> = IntMap::default();

    for (eid, data) in &log.events {
        for oid in &data.omap {
            if !ocdg.node_attributes.contains_key(oid) {
                let new_node = ocdg.net.add_node(*oid);
                ocdg.object_map.insert(log.object_map.get_by_right(&oid).expect("This cannot occur").to_owned(), *oid);
                ocdg.init_object_key(*oid);
                ocdg.inodes.entry(*oid).or_insert(new_node);
                let curr_obj = &log.objects[oid];
                ocdg.node_attributes.entry(*oid).or_default().node_type = curr_obj.obj_type.to_owned();

            }
            neighbours.entry(*oid).or_default().extend(&log.events.get(&eid).unwrap().omap);
        }

    }

    let new_edges: Vec<(usize, usize, EventAdd, Relations)> = ocdg.inodes.par_iter()
                           .map(|(oid, _)| whole_instance_edges(&log, &ocdg, oid, &neighbours, &rel_whole, &rel_inst))
                           .flatten()
                           .collect();

    let mut ev_added: AHashSet<usize> = AHashSet::new();
    for edge in new_edges {
        match edge.2.clone() {
            EventAdd::SINGLE(ev) => {
                ev_added.insert(ev);
            },
            EventAdd::MULTI(evs) => {
                for ev in evs {
                    ev_added.insert(ev);
                    
                }
            }
        }

        ocdg.apply_new_edges((edge.0, edge.1), edge.2, edge.3);
    }
    
    // add event mappings
    for ev in ev_added {
        ocdg.event_map.insert(log.event_map.get_by_right(&ev).expect("This cannot fail ever").to_owned(), ev);
    }
    ocdg
}


fn whole_instance_edges(log: &Ocel, ocdg:&Ocdg, oid1: &usize, neighs: &IntMap<usize, IntSet<usize>>, rel_whole: &Vec<&Relations>, rel_inst: &Vec<&Relations>) -> Vec<(usize, usize, EventAdd, Relations)> {
        // println!("{:?} reporting in!", &oid1);
        let mut oid_edges: Vec<(usize, usize, EventAdd, Relations)> = vec![];
        for rel in rel_whole {
            oid_edges.extend(rel.execute_whole(&log, &ocdg, neighs, *oid1));
        }
        for oid2 in neighs.get(oid1).unwrap() {
            if oid1 != oid2 {
                for rel in rel_inst {
                    oid_edges.extend(rel.execute(&log, &ocdg, *oid1, *oid2));
                }
            }

        }
        oid_edges

}

fn intersection_count_sorted_vec(a: &[usize], b: &[usize]) -> IntSet<usize> {
    let mut intersected: IntSet<usize> = IntSet::default();
    let mut b_iter = b.iter();
    if let Some(mut current_b) = b_iter.next() {
        for current_a in a {
            while current_b < current_a {
                current_b = match b_iter.next() {
                    Some(current_b) => current_b,
                    None => return intersected,
                };
            }
            if current_a == current_b {
                intersected.insert(*current_a);
            }
        }
    }
    intersected
}
