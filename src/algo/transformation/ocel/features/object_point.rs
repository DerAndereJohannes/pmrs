use std::collections::HashMap;
use std::str::FromStr;
use ahash::AHashSet;
use petgraph::EdgeDirection::Outgoing;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use serde_json::Value;

use crate::objects::ocel::Ocel;
use crate::objects::ocdg::Ocdg;

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

