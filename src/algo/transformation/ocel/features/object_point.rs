use std::str::FromStr;
use ahash::AHashSet;
use petgraph::EdgeDirection::Outgoing;
use itertools::Itertools;
use petgraph::graph::NodeIndex;

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

pub fn activity_existence<'a>(ocel: &Ocel, ocdg: &Ocdg, oid: usize) -> Vec<&'a str> {
    let curr_oid: NodeIndex = ocdg.inodes[&oid];
    let activities: &AHashSet<String> = &ocel.activities;

    let mut activity_vec: Vec<&str> = vec![];
    
    activity_vec

}


