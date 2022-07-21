use std::collections::HashMap;
use ahash::{AHashSet, AHashMap};
use chrono::Duration;
use petgraph::EdgeDirection::Outgoing;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use polars::prelude::{Series, NamedFromOwned, DataFrame};
use serde_json::Value;
use strum::{EnumString, Display, IntoStaticStr};
use rayon::prelude::*;

use crate::objects::linker::link_objects;
use crate::objects::ocel::Ocel;
use crate::objects::ocdg::{Ocdg, Relations};
use super::operator::Operator;

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
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


pub struct ObjectPointConfig<'a> {
    pub ocel: &'a Ocel,
    pub ocdg: &'a Ocdg,
    pub params: &'a HashMap<ObjectPoint, Option<Value>>

}

pub fn object_point_features(config: ObjectPointConfig) -> DataFrame {
    let object_linker = link_objects(&config.ocel.object_map, &config.ocdg.object_map);
    let entity_order: Vec<usize> = object_linker.left_values().map(|v| **v).collect();
    let mut string_oids: Series = entity_order.iter().map(|oid| config.ocel.object_map.get_by_right(oid).unwrap().to_owned()).collect();
    string_oids.rename("oids");
    let mut series_vec: Vec<Series> = vec![string_oids];

    for (feature, _params) in config.params {

        match feature {
            ObjectPoint::UniqueNeighborCount => {
                let feature_values: Vec<(usize, usize)> = entity_order.par_iter()
                                                            .enumerate()
                                                            .map(|(index, log_oid)| {
                                                                (index, unique_neighbor_count(config.ocdg, log_oid))
                                                            })
                                                            .collect();
                
                let mut feature_vector: Vec<u32> = Vec::with_capacity(feature_values.len());
                feature_values.iter().for_each(|(index, value)| feature_vector.insert(*index, (*value).try_into().unwrap()));
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
                
            },
            ObjectPoint::ActivityExistence => {
                let feature_values: Vec<(usize, usize, u8)> = entity_order.par_iter()
                                                            .enumerate()
                                                            .map(|(index, log_oid)| {
                                                                activity_existence(config.ocel, *log_oid).iter().enumerate().map(|(actid, res)| (index, actid, *res)).collect::<Vec<(usize, usize, u8)>>()
                                                            })
                                                            .flatten()
                                                            .collect();

                let mut feature_vector: Vec<Vec<u8>> = vec![Vec::with_capacity(entity_order.len()); config.ocel.activities.len()];
                feature_values.iter().for_each(|(oid, act_id, value)| feature_vector.get_mut(*act_id).expect("This can't fail").insert(*oid, *value));

                config.ocel.activities.iter().enumerate().for_each(|(index, act_name)| {
                                                                        series_vec.push(Series::from_vec(format!("{}:{}", feature, act_name).as_str(), feature_vector[index].clone()));
                                                                    });

            },
            ObjectPoint::ActivityExistenceCount => {
                let feature_values: Vec<(usize, usize, usize)> = entity_order.par_iter()
                                                            .enumerate()
                                                            .map(|(index, log_oid)| {
                                                                activity_existence_count(config.ocel, *log_oid).iter().enumerate().map(|(actid, res)| (index, actid, *res)).collect::<Vec<(usize, usize, usize)>>()
                                                            })
                                                            .flatten()
                                                            .collect();

                let mut feature_vector: Vec<Vec<u32>> = vec![Vec::with_capacity(entity_order.len()); config.ocel.activities.len()];
                feature_values.iter().for_each(|(oid, act_id, value)| feature_vector.get_mut(*act_id).expect("This can't fail").insert(*oid, (*value).try_into().unwrap()));

                config.ocel.activities.iter().enumerate().for_each(|(index, act_name)| {
                                                                        series_vec.push(Series::from_vec(format!("{}:{}", feature, act_name).as_str(), feature_vector[index].clone()))
                                                                    });

            },
            ObjectPoint::ObjectLifetime => {
                let feature_values: Vec<(usize, Duration)> = entity_order.par_iter()
                                                            .enumerate()
                                                            .map(|(index, log_oid)| {
                                                                (index, object_lifetime(config.ocel, log_oid))
                                                            })
                                                            .collect();

                let mut feature_vector: Vec<i64> = Vec::with_capacity(feature_values.len());
                feature_values.iter().for_each(|(index, value)| feature_vector.insert(*index, match value.num_nanoseconds() {Some(v) => {v}, None => {0}}));
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
            },
            ObjectPoint::ObjectUnitSetRatio => {
                let feature_values: Vec<(usize, f64)> = entity_order.par_iter()
                                                            .enumerate()
                                                            .map(|(index, log_oid)| {
                                                                (index, object_unit_set_ratio(config.ocel, log_oid))
                                                            })
                                                            .collect();

                let mut feature_vector: Vec<f64> = Vec::with_capacity(feature_values.len());
                feature_values.iter().for_each(|(index, value)| feature_vector.insert(*index, *value));
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
            },
            ObjectPoint::ObjectEventInteractionOperator => {
                let feature_values: Vec<(usize, f64)> = entity_order.par_iter()
                                                            .enumerate()
                                                            .map(|(index, log_oid)| {
                                                                (index, object_average_event_interaction(config.ocel, log_oid))
                                                            })
                                                            .collect();

                let mut feature_vector: Vec<f64> = Vec::with_capacity(feature_values.len());
                feature_values.iter().for_each(|(index, value)| feature_vector.insert(*index, *value));
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
            }
            _ => {}
        }
    }
    DataFrame::new(series_vec).unwrap()
}

pub fn unique_neighbor_count(ocdg: &Ocdg, oid: &usize) -> usize {
    let curr_oid: NodeIndex = ocdg.inodes[oid];
    ocdg.net.neighbors_directed(curr_oid, Outgoing).into_iter()
                                                   .unique()
                                                   .count()
}

pub fn activity_existence(log: &Ocel, oid: usize) -> Vec<u8> {
    let oe_activities: AHashSet<&String> = AHashSet::from_iter(log.objects[&oid].events.iter()
                                            .map(|oe| &log.events[&oe].activity));
    log.activities.iter()
                   .map(|act| {if oe_activities.contains(act) {1} else {0}})
                   .collect_vec()
}


pub fn activity_existence_count(log: &Ocel, oid: usize) -> Vec<usize> {
    let oe_activities: HashMap<&String, usize> = log.objects[&oid].events.iter()
                                                                         .map(|oe| &log.events[&oe].activity)
                                                                         .counts();
    log.activities.iter()
              .map(|act| {match oe_activities.get(act) {
                            Some(v) => *v,
                            None => 0
                        }})
              .collect_vec()
}

pub fn activity_value_operator(log: &Ocel, oid: usize, attr: String, op: Operator) -> f64 {
    op.execute(log.objects[&oid].events.iter()
                            .filter(|oe| !log.events[&oe].vmap.contains_key(&attr))
                            .map(|oe| match &log.events[&oe].vmap[&attr] {
                                        Value::Number(v) => v.as_f64().unwrap(),
                                        _ => 0.0
                                    })).unwrap()

}

pub fn object_type_relations_value_operator() {todo!();}

pub fn object_lifetime(log: &Ocel, oid: &usize) -> Duration {
    if let Some(node) = log.objects.get(oid) {
        let initial = node.events.first().unwrap();
        let end = node.events.last().unwrap();

        if log.events.contains_key(&initial) && log.events.contains_key(&end) {
            return log.events[&end].timestamp - log.events[&initial].timestamp;
        }
    }
    Duration::zero()
}

pub fn object_unit_set_ratio(log: &Ocel, oid: &usize) -> f64 {
    if let Some(node) = log.objects.get(oid) {
        let unitset = node.events.iter()
                          .map(|ev| {
                              if log.events.contains_key(ev) {
                                  for oid2 in &log.events[ev].omap {
                                        if oid != oid2 && log.objects[oid].obj_type == log.objects[oid2].obj_type {
                                            return 0;
                                        }
                                  }
                              } else {
                                return 0;
                              }
                              1
                          }).fold(0, |accum, item| accum + item);

        return unitset as f64 / node.events.len() as f64
    }
    0.0
}

pub fn object_average_event_interaction(log: &Ocel, oid: &usize) -> f64 {
    if let Some(node) = log.objects.get(oid) {
        let interaction = node.events.iter()
                                     .map(|ev| {
                                        if log.events.contains_key(ev) {
                                            return log.events[ev].omap.len() - 1;
                                        }
                                        0})
                                     .fold(0, |accum, item| accum + item);

        return interaction as f64 / node.events.len() as f64

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

pub fn object_events_directly_follows(log: &Ocel, oid: &usize) -> AHashMap<String, AHashMap<String, usize>> {
    let mut df: AHashMap<String, AHashMap<String, usize>> = AHashMap::default();
    if let Some(obj) = log.objects.get(oid) {
        (0..obj.events.len() - 1).into_iter()
                                 .for_each(|i| {
                                    let src = &log.events[&obj.events[i]].activity;
                                    let tar = &log.events[&obj.events[i+1]].activity;
                                    let df_srctar = df.entry(src.to_owned())
                                      .or_insert(AHashMap::default())
                                      .entry(tar.to_owned())
                                      .or_insert(0);

                                    *df_srctar += 1;
                                 });
    }
    df
}

pub fn object_wait_time(log: &Ocel, oid: &usize, act1: &str, act2: &str) -> Duration {
    let mut time_diff = Duration::zero();
    if let Some(obj) = log.objects.get(oid) {
        let mut ev1: usize = usize::MAX;
        let mut ev2: usize = usize::MAX;
        obj.events.iter().rev().for_each(|item|{
            if let Some(curr) = log.events.get(item) {
                if ev2 == usize::MAX {
                    if curr.activity == act2 {
                        ev2 = *item; 
                    } 
                } else if ev1 == usize::MAX {
                    if curr.activity == act1 {
                        ev1 = *item;
                    }
                } else if time_diff == Duration::zero() {
                    time_diff = log.events[&ev2].timestamp - log.events[&ev1].timestamp;
                }
            }
        });
    }
    time_diff
}

pub fn object_oe_root(log: &Ocel, oid: &usize) -> bool {
    if let Some(obj) = log.objects.get(oid) {
        if let Some(root_ev) = obj.events.first() {
            let root1 = &log.events[root_ev];
            for oid2 in root1.omap.iter() {
                if let Some(other) = log.objects.get(&oid2) {
                    if let Some(root_ev2) = other.events.first() {
                        let root2 = &log.events[root_ev2];
                        if root1.timestamp > root2.timestamp {
                            return false;
                        }

                    }
                }
                
            }
        
        }
        true
    } else {
        false
    }
    
}


pub fn object_oe_leaf(log: &Ocel, oid: &usize) -> bool {
    if let Some(obj) = log.objects.get(oid) {
        if let Some(leaf_ev) = obj.events.last() {
            let leaf1 = &log.events[leaf_ev];
            for oid2 in leaf1.omap.iter() {
                if let Some(other) = log.objects.get(&oid2) {
                    if let Some(leaf_ev2) = other.events.last() {
                        let leaf2 = &log.events[leaf_ev2];
                        if leaf1.timestamp < leaf2.timestamp {
                            return false;
                        }

                    }
                }
                
            }
        
        }
        true
    } else {
        false
    }
    
}


pub fn object_direct_rel_count(ocdg: &Ocdg, oid: &usize, rel: &Relations) -> usize {
    if let Some(obj) = ocdg.inodes.get(oid) {
        let neighs = ocdg.net.neighbors_directed(*obj, Outgoing);
        return neighs.enumerate().map(|(_i, neigh)| {
            let neigh_id = &ocdg.net[neigh];
            let conn = ocdg.irels.get(oid).unwrap().get(neigh_id).unwrap();
            if conn.contains_key(&(rel.relation_index())) {
                1
            } else {
                0
            }
        }).fold(0 as usize, |accum, item| accum + item);
        
    }
0
}

pub fn object_subgraph_count() {todo!()}
