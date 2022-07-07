use std::collections::HashMap;
use std::str::FromStr;
use ahash::AHashSet;
use chrono::Duration;
use petgraph::EdgeDirection::Outgoing;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use serde_json::Value;

use crate::objects::ocel::Ocel;
use crate::objects::ocdg::Ocdg;
// use super::operator::Operator;

pub enum ObjectPoint {
    UniqueNeighborCount,
    ActivityExistence,
    ActivityExistenceCount,
    ActivityValueOperator,
    ObjectTypeRelationsValueOperator,
    ObjectLifetime,
    ObjectUnitSetRatio,
    ObjectEventInteractionOperator,
    ObjectTypeInteraction,
    ObjectEventsDirectlyFollows,
    ObjectWaitTime,
    ObjectStartEnd,
    DirectRelationCount,
    SubgraphExistenceCount
}


impl FromStr for ObjectPoint {
    type Err = ();

    fn from_str(feature: &str) -> Result<ObjectPoint, Self::Err> {
        match feature {
            "UniqueNeighborCount" => Ok(ObjectPoint::UniqueNeighborCount),
            "ActivityExistence" => Ok(ObjectPoint::ActivityExistence),
            "ActivityExistenceCount" => Ok(ObjectPoint::ActivityExistenceCount),
            "ActivityValueOperator" => Ok(ObjectPoint::ActivityValueOperator),
            "ObjectTypeRelationsValueOperator" => Ok(ObjectPoint::ObjectTypeRelationsValueOperator),
            "ObjectLifetime" => Ok(ObjectPoint::ObjectLifetime),
            "ObjectUnitSetRatio" => Ok(ObjectPoint::ObjectUnitSetRatio),
            "ObjectEventInteractionOperator" => Ok(ObjectPoint::ObjectEventInteractionOperator),
            "ObjectTypeInteraction" => Ok(ObjectPoint::ObjectTypeInteraction),
            "ObjectEventsDirectlyFollows" => Ok(ObjectPoint::ObjectEventsDirectlyFollows),
            "ObjectWaitTime" => Ok(ObjectPoint::ObjectWaitTime),
            "ObjectStartEnd" => Ok(ObjectPoint::ObjectStartEnd),
            "DirectRelationCount" => Ok(ObjectPoint::DirectRelationCount),
            "SubgraphExistenceCount" => Ok(ObjectPoint::SubgraphExistenceCount),
            _ => Err(()),
        }
    }
}


pub fn unique_neighbor_count(ocdg: &Ocdg, oid: usize) -> usize {
    let curr_oid: NodeIndex = ocdg.inodes[&oid];
    ocdg.net.neighbors_directed(curr_oid, Outgoing).into_iter()
                                                   .unique()
                                                   .count()
}

pub fn activity_existence(ocel: &Ocel, ocdg: &Ocdg, oid: usize) -> Vec<u8> {
    let oe_activities: AHashSet<&String> = AHashSet::from_iter(ocdg.node_attributes[&oid].object_events.iter()
                                            .map(|oe| &ocel.events[&oe].activity));
    ocel.activities.iter()
                   .map(|act| {if oe_activities.contains(act) {1} else {0}})
                   .collect_vec()
}


pub fn activity_existence_count(ocel: &Ocel, ocdg: &Ocdg, oid: usize) -> Vec<usize> {
    let oe_activities: HashMap<&String, usize> = ocdg.node_attributes[&oid].object_events.iter()
                                                                                          .map(|oe| &ocel.events[&oe].activity)
                                                                                          .counts();
    ocel.activities.iter()
              .map(|act| {match oe_activities.get(act) {
                            Some(v) => *v,
                            None => 0
                        }})
              .collect_vec()
}

pub fn activity_value_operator(ocel: &Ocel, ocdg: &Ocdg, oid: usize, attr: String) -> f64 {
    ocdg.node_attributes[&oid].object_events.iter()
                                            .filter(|oe| !ocel.events[&oe].vmap.contains_key(&attr))
                                            .map(|oe| match &ocel.events[&oe].vmap[&attr] {
                                                        Value::Number(v) => v.as_f64().unwrap(),
                                                        _ => 0.0
                                            })
                                            .sum::<f64>() //change for operator

}

pub fn object_type_relations_value_operator() {todo!();}

pub fn object_lifetime(ocel: &Ocel, ocdg: &Ocdg, oid: &usize) -> Duration {
    if let Some(node) = ocdg.node_attributes.get(oid) {
        let initial = node.object_events.first().unwrap();
        let end = node.object_events.last().unwrap();

        if ocel.events.contains_key(&initial) && ocel.events.contains_key(&end) {
            return ocel.events[&end].timestamp - ocel.events[&initial].timestamp;
        }
    }
    Duration::zero()
}

pub fn object_unit_set_ratio(ocel: &Ocel, ocdg: &Ocdg, oid: &usize) -> f64 {
    if let Some(node) = ocdg.node_attributes.get(oid) {
        let unitset = node.object_events.iter()
                          .map(|ev| {
                              if ocel.events.contains_key(ev) {
                                  for oid2 in &ocel.events[ev].omap {
                                        if oid != oid2 && ocel.objects[oid].obj_type == ocel.objects[oid2].obj_type {
                                            return 0;
                                        }
                                  }
                              } else {
                                return 0;
                              }
                              1
                          }).fold(0, |accum, item| accum + item);

        return (unitset / node.object_events.len()) as f64
    }
    0.0
}

pub fn object_average_event_interaction(ocel: &Ocel, ocdg: &Ocdg, oid: &usize) -> f64 {
    if let Some(node) = ocdg.node_attributes.get(oid) {
        let interaction = node.object_events.iter()
                                            .map(|ev| {
                                                if ocel.events.contains_key(ev) {
                                                    return ocel.events[ev].omap.len() - 1;
                                                }
                                                0
                                            }).fold(0, |accum, item| accum + item);

        return (interaction / node.object_events.len()) as f64

    }
    0.0
}

pub fn object_type_interaction(ocdg: &Ocdg, oid: &usize, otype: &str) -> usize {
    if let Some(node) = ocdg.inodes.get(oid) {
        let neighs = ocdg.net.neighbors_directed(*node, Outgoing);
        return neighs.map(|oid2| {if oid != &ocdg.net[oid2] 
                              && otype == ocdg.node_attributes[&ocdg.net[oid2]].node_type {1} else {0}})
              .fold(0, |accum, item| accum + item);

    }
    0
    
}

pub fn object_events_directly_follows() {todo!()}

pub fn object_wait_time() {todo!()}

pub fn object_start_end() {todo!()}

pub fn object_direct_rel_count() {todo!()}

pub fn object_subgraph_count() {todo!()}
