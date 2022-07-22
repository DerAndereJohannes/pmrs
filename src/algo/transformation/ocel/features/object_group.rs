use std::collections::HashMap;

use ahash::AHashSet;
use petgraph::{EdgeDirection::{Incoming, Outgoing}, graph::NodeIndex};
use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::{ocel::Ocel, ocdg::{Ocdg, Relations}};

use super::operator::Operator;

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum ObjectGroup {
    ObjectTypeCount,
    ObjectTypeAttrOperator,
    OtOtInteractions,
    RootNodeCount,
    LeafNodeCount,
    SeparationComplexityOperator
}


pub struct ObjectGroupConfig<'a> {
    pub ocel: &'a Ocel,
    pub ocdg: &'a Ocdg,
    pub params: &'a HashMap<ObjectGroup, Option<Value>>

}


pub fn object_type_count(log: &Ocel, otype: &str) -> usize {
    log.objects.iter().filter(|(_, values)| values.obj_type == otype).count() 
}

pub fn object_type_attr_operator(log: &Ocel, otype: &str, attr: &str, op: Operator) -> f64 {
    op.execute(log.objects.iter()
                          .filter(|(_, values)| values.obj_type == otype && values.ovmap.contains_key(attr))
                          .map(|(_, values)| values.ovmap[attr].as_f64().unwrap())).unwrap()
}

pub fn ot_ot_interactions(ocdg: &Ocdg, ot1: &str, ot2: &str, relation: &Relations) -> u64 {
   ocdg.inodes.iter()
              .filter(|(oid, _node)| ocdg.node_attributes.get(oid).unwrap().node_type == ot1 && ocdg.irels.contains_key(oid))
              .map(|(oid, _node)| ocdg.irels[oid].iter().filter(|(oid2, rel)| ocdg.node_attributes.get(oid2).unwrap().node_type == ot2 && rel.contains_key(&relation.relation_index())).count() as u64)
              .sum() 
}

pub fn root_node_count(ocdg: &Ocdg, otype: &str) -> usize {
    ocdg.inodes.iter()
               .filter(|(oid, _node)| ocdg.node_attributes.get(oid).unwrap().node_type == otype)
               .filter(|(_oid, node)| ocdg.net.neighbors_directed(**node, Incoming).count() == 0)
               .count()
}

pub fn leaf_node_count(ocdg: &Ocdg, otype: &str) -> usize {
    ocdg.inodes.iter()
               .filter(|(oid, _node)| ocdg.node_attributes.get(oid).unwrap().node_type == otype)
               .filter(|(_oid, node)| ocdg.net.neighbors_directed(**node, Outgoing).count() == 0)
               .count()
}

pub fn separation_complexity_operator(ocdg: &Ocdg, op: Operator) -> f64 {
    let mut curr_nodes: AHashSet<NodeIndex> = AHashSet::from_iter(ocdg.inodes.iter()
                                                                              .filter(|(_oid, node)| ocdg.net.neighbors_directed(**node, Incoming).count() == 0)
                                                                              .map(|(_oid, node)| node.to_owned()));
    let mut complexity_vec: Vec<usize> = vec![curr_nodes.len()];

    while curr_nodes.len() != 0 {
        let new_nodes: AHashSet<NodeIndex> = AHashSet::from_iter(curr_nodes.iter()
                                                                                .map(|node| ocdg.net.neighbors_directed(*node, Outgoing).into_iter()
                                                                                                                                         .collect::<Vec<NodeIndex>>())
                                                                                .flatten());
        
        complexity_vec.push(new_nodes.len());
        curr_nodes = new_nodes;
    }

    op.execute(complexity_vec.iter().map(|item| *item as f64)).unwrap()
 
}
