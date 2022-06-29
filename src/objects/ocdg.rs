use std::{collections::hash_map::Entry, vec};
use petgraph::{graph::{DiGraph, NodeIndex, EdgeIndex, Neighbors}, EdgeDirection::Outgoing};
use nohash_hasher::{IntSet, IntMap};
use array_tool::vec::Intersect;
use rayon::prelude::*;

use super::ocel::Ocel;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Relations {
    INTERACTS = 1,
    COLIFE = 2,
    COBIRTH = 3,
    CODEATH = 4,
    DESCENDANTS = 5,
    INHERITANCE = 6,
    CONSUMES = 7,
    SPLIT = 8,
    MERGE = 9,
    MINION = 10,
    PEELER = 11,
    ENGAGES = 12
}

impl Relations {
    fn relation_type(&self) -> u8 {
        match self {
            Relations::INTERACTS => {1},
            Relations::DESCENDANTS => {1},
            Relations::SPLIT => 3,
            _ => {2}
            
        }
    }

    fn relation_index(&self) -> u8 {
        match self {
            Relations::INTERACTS => 1,
            Relations::COLIFE => 2,
            Relations::COBIRTH => 3,
            Relations::CODEATH => 4,
            Relations::DESCENDANTS => 5,
            Relations::INHERITANCE => 6,
            Relations::CONSUMES => 7,
            Relations::SPLIT => 8,
            Relations::MERGE => 9,
            Relations::MINION => 10,
            Relations::PEELER => 11,
            Relations:: ENGAGES => 12
        }
    }

    
    fn execute_primitive(&self, ocdg: &Ocdg, oid1: usize, oid2: usize, eid: usize) -> Vec<(usize, usize, EventAdd, Relations)> {
        let mut to_add: Vec<(usize, usize, EventAdd, Relations)> = Vec::new();
        match self {
            Relations::INTERACTS => {
                to_add.push((oid1, oid2, EventAdd::SINGLE(eid), Relations::INTERACTS));
            },
            Relations::DESCENDANTS => {
                let src_oe = &ocdg.node_attributes.get(&oid1).unwrap().object_events;
                let tar_oe_test = &ocdg.node_attributes.get(&oid2);

                match tar_oe_test {
                    Some(tar) => {
                        if src_oe.len() > 1 && tar.object_events.len() == 1 {
                            to_add.push((oid1, oid2, EventAdd::SINGLE(eid), Relations::DESCENDANTS));
                        }
                    },
                    None => {
                        if src_oe.len() > 1 {
                            to_add.push((oid1, oid2, EventAdd::SINGLE(eid), Relations::DESCENDANTS));
                        }
                    }
                }
                // println!("{:?} {:?} {:?} {:?} {:?}", src_oe, tar_oe_test, oid1, oid2, &to_add);
            },
            _ => {},
        }
        to_add
    }

    fn execute_whole(&self, _log: &Ocel, ocdg: &Ocdg, oid1: usize, neighbors: &Neighbors<usize>) -> Vec<(usize, usize, EventAdd, Relations)> {
        let mut to_add: Vec<(usize, usize, EventAdd, Relations)> = Vec::new();
        let src_oe = &ocdg.node_attributes.get(&oid1).unwrap().object_events;
        let src_type = &ocdg.node_attributes.get(&oid1).unwrap().node_type;
            match self {
                Relations::SPLIT => {
                    let mut conforming_oid: IntSet<usize> = IntSet::default();
                    let src_e = src_oe.last().unwrap();
                    let mut neighbor_walker = neighbors.detach();
                    while let Some(neigh) = neighbor_walker.next_node(&ocdg.net) {
                        let oid2 = ocdg.net.node_weight(neigh).unwrap();
                        let neigh_oe = &ocdg.node_attributes.get(&oid2).unwrap().object_events;
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
        let src_oe = &ocdg.node_attributes.get(&oid1).unwrap().object_events;
        let tar_oe = &ocdg.node_attributes.get(&oid2).unwrap().object_events;
        let src_type = &ocdg.node_attributes.get(&oid1).unwrap().node_type;
        let tar_type = &ocdg.node_attributes.get(&oid2).unwrap().node_type;
        
        match self {
            Relations::COLIFE => { // one time
                if src_oe == tar_oe {
                    let e_set: IntSet<usize> = IntSet::from_iter(src_oe.to_owned());
                    to_add.push((oid1, oid2, EventAdd::MULTI(e_set), Relations::COLIFE));
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
                       let common_events: Vec<_> = src_oe.intersect(tar_oe.to_vec()); 
                       if common_events.len() == tar_oe.len() {
                            to_add.push((oid1, oid2, EventAdd::MULTI(IntSet::<usize>::from_iter(common_events)), Relations::MINION));
                       }
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


#[derive(Debug)]
pub enum EventAdd {
    SINGLE(usize),
    MULTI(IntSet<usize>)
}


#[derive(Debug, Default)]
pub struct NodeInfo<'a> {
    pub node_type: &'a str,
    pub object_events: Vec<usize>
}

pub struct Ocdg<'a> {
    pub net: DiGraph<usize, usize>,
    pub edge_attributes: IntMap<usize, NodeInfo<'a>>,
    pub node_attributes: IntMap<usize, NodeInfo<'a>>,
    pub inodes: IntMap<usize, NodeIndex>,
    pub iedges: IntMap<usize, IntMap<usize, EdgeIndex>>,
    pub irels: IntMap<usize, IntMap<usize,IntMap<usize, IntSet<usize>>>>
}

impl Default for Ocdg<'_> {
    fn default() -> Self {
        Self{
            net: DiGraph::<usize, usize>::new(),
            edge_attributes: IntMap::default(),
            node_attributes: IntMap::default(),
            inodes: IntMap::default(),
            iedges: IntMap::default(),
            irels: IntMap::default()
        }
    }
}


impl<'a> Ocdg<'a> {

    fn init_object_key(&mut self, oid: usize) {
        self.node_attributes.insert(oid, NodeInfo::default());
    }


    fn add_eid_to_oe(&mut self, oid: usize, eid: usize) {
        let oe_entry: &mut Vec<usize> = &mut self.node_attributes.entry(oid).or_default().object_events;
        oe_entry.push(eid);

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

pub fn generate_ocdg<'a>(log: &'a Ocel, relations: &'a Vec<Relations>) -> Ocdg<'a> {
    let mut ocdg: Ocdg = Ocdg::default();
    let mut new_edges: Vec<(usize, usize, EventAdd, Relations)> = vec![]; 
    // let rel_prim: Vec<_> = relations.iter().filter(|r| r.relation_type() == 1).collect();
    let rel_inst: Vec<_> = relations.iter().filter(|r| r.relation_type() == 2).collect();
    let rel_whole: Vec<_> = relations.iter().filter(|r| r.relation_type() == 3).collect();

    for (eid, data) in &log.events {
        for oid in &data.omap {
            if !ocdg.node_attributes.contains_key(oid) {
                let new_node = ocdg.net.add_node(*oid);
                ocdg.init_object_key(*oid);
                ocdg.inodes.entry(*oid).or_insert(new_node);
                let curr_obj = &log.objects[oid];
                ocdg.node_attributes.entry(*oid).or_default().node_type = &curr_obj.obj_type;
                ocdg.node_attributes.entry(*oid).or_default().object_events = vec![];

            }
            ocdg.add_eid_to_oe(*oid, *eid);
        }
        new_edges.extend(
            data.omap.iter()
                     .map(|oid1| {
                        let mut to_add: Vec<(usize, usize, EventAdd, Relations)> = vec![];
                        for oid2 in &data.omap {
                            if oid1 != oid2 {
                                to_add.extend(Relations::INTERACTS.execute_primitive(&ocdg, *oid1, *oid2, *eid));
                                to_add.extend(Relations::DESCENDANTS.execute_primitive(&ocdg, *oid1, *oid2, *eid));
                            }
                        }
                        to_add
                 })
                     .flatten()
                     .collect::<Vec<(usize, usize, EventAdd, Relations)>>());
    }

    for edge in new_edges {
        ocdg.apply_new_edges((edge.0, edge.1), edge.2, edge.3);
    }

    new_edges = ocdg.inodes.par_iter()
                           .map(|(oid, node)| whole_instance_edges(&log, &ocdg, oid, node, &rel_whole, &rel_inst))
                           .flatten()
                           .collect();
    
    for edge in new_edges {
        ocdg.apply_new_edges((edge.0, edge.1), edge.2, edge.3);
    }


    ocdg
}


fn whole_instance_edges(log: &Ocel, ocdg:&Ocdg, oid1: &usize, node: &NodeIndex, rel_whole: &Vec<&Relations>, rel_inst: &Vec<&Relations>) -> Vec<(usize, usize, EventAdd, Relations)> {
        let mut oid_edges: Vec<(usize, usize, EventAdd, Relations)> = vec![];
        let neighborhood = ocdg.net.neighbors_directed(*node, Outgoing);
        for rel in rel_whole {
            oid_edges.extend(rel.execute_whole(&log, &ocdg, *oid1, &neighborhood));
        }
        let mut neighbor_walker = neighborhood.detach();
        while let Some(neigh) = &neighbor_walker.next_node(&ocdg.net){
            let oid2 = ocdg.net.node_weight(*neigh).unwrap();
            if ocdg.irels.get(&oid1).unwrap().get(&*oid2).unwrap().len() > 0 {
                for rel in rel_inst {
                    oid_edges.extend(rel.execute(&log, &ocdg, *oid1, *oid2));
                }
            }

        }
        oid_edges

}
