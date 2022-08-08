use std::collections::HashMap;
use std::str::FromStr;
use ahash::{AHashSet, AHashMap};
use chrono::Duration;
use petgraph::EdgeDirection::Outgoing;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use polars::prelude::{Series, NamedFromOwned, DataFrame, NamedFrom};
use serde_json::Value;
use strum::{EnumString, Display, IntoStaticStr};
use rayon::prelude::*;

use crate::objects::ocel::Ocel;
use crate::objects::ocdg::{Ocdg, Relations};
use super::operator::Operator;

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq, Debug)]
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
    ObjectDirectRelationCount,
    SubgraphExistenceCount
}


pub struct ObjectPointConfig<'a> {
    pub ocel: &'a Ocel,
    pub ocdg: &'a Ocdg,
    pub params: &'a Vec<(ObjectPoint, Option<Value>)>

}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}


pub fn object_point_features(config: ObjectPointConfig) -> DataFrame {
    let obj_str_vec: Vec<&str> = config.ocel.objects.keys().map(|oid| config.ocel.object_map.get_by_right(oid).unwrap().as_str()).collect();

    let mut series_vec: Vec<Series> = vec![Series::new("oids", obj_str_vec.clone())];

    for (feature, params) in config.params {
        match feature {
            ObjectPoint::UniqueNeighborCount => {
                let mut feature_vector: Vec<u64> = vec![0;obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  *v = unique_neighbor_count(config.ocdg, config.ocdg.object_map.get_by_left(obj_str_vec[i]).unwrap()) as u64
                              });
                
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
                
            },
            ObjectPoint::ActivityExistence => {
                let mut feature_vector: Vec<Vec<u8>> = vec![vec![0;config.ocel.activities.len()];obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  *v = activity_existence(config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap());
                              });
                
                for (v, act) in transpose(feature_vector).iter().zip(&config.ocel.activities) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:exists", feature, act).as_str(), v));
                }
            },
            ObjectPoint::ActivityExistenceCount => {
                let mut feature_vector: Vec<Vec<u64>> = vec![vec![0;config.ocel.activities.len()];obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  *v = activity_existence_count(config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap()).iter().map(|c| *c as u64).collect();
                              });
                
                for (v, act) in transpose(feature_vector).iter().zip(&config.ocel.activities) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, act).as_str(), v));
                }
            },
            ObjectPoint::ObjectLifetime => {
                let mut feature_vector: Vec<i64> = vec![0;obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  *v = object_lifetime(config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap()).num_milliseconds();
                              });
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
            },
            ObjectPoint::ObjectUnitSetRatio => {
                let mut feature_vector: Vec<f64> = vec![0.0;obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  *v = object_unit_set_ratio(config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap());
                              });
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
            },
            ObjectPoint::ObjectEventInteractionOperator => {
                let mut feature_vector: Vec<f64> = vec![0.0;obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  *v = object_average_event_interaction(config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap());
                              });
                series_vec.push(Series::from_vec(feature.into(), feature_vector));
            },
            ObjectPoint::ActivityValueOperator => {
                if let Some(f_params) = params {
                    let attr: Option<&Value> = f_params.get("attribute");
                    let op: Option<&Value> = f_params.get("operator");
                    if let (Some(attr_valid), Some(op_valid)) = (attr, op) {
                        let attr_str = attr_valid.as_str().unwrap();
                        let op_enum = Operator::from_str(op_valid.as_str().unwrap()).unwrap();
                        let mut feature_vector: Vec<f64> = vec![0.0; obj_str_vec.len()];
                        feature_vector.par_iter_mut()
                                      .enumerate()
                                      .for_each(|(i, v)| {
                                        *v = activity_value_operator(&config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap(), attr_str, &op_enum);
                                      });
                        series_vec.push(Series::from_vec(format!("{:?}:{:?}:{:?}", feature, attr_str, op_enum).as_str(), feature_vector));
                    }
                } 
            },
            ObjectPoint::ObjectTypeInteraction => {
                if let Some(f_params) = params {
                    if let Some(otype_valid) = f_params.get("object_type") {
                        let otype_str = otype_valid.as_str().unwrap();
                        let mut feature_vector: Vec<u64> = vec![0;obj_str_vec.len()];
                        feature_vector.par_iter_mut()
                                      .enumerate()
                                      .for_each(|(i, v)| {
                                        *v = object_type_interaction(&config.ocdg, config.ocdg.object_map.get_by_left(obj_str_vec[i]).unwrap(), otype_str) as u64;
                                      });

                        series_vec.push(Series::from_vec(format!("{:?}:{:?}", feature, otype_str).as_str(), feature_vector));
                    }
                } 
            },
            ObjectPoint::ObjectWaitTime => {
                if let Some(f_params) = params {
                    let act1: Option<&Value> = f_params.get("activity_src");
                    let act2: Option<&Value> = f_params.get("activity_tar");
                    if let (Some(act1_valid), Some(act2_valid)) = (act1, act2) {
                        let act1_str = act1_valid.as_str().unwrap();
                        let act2_str = act2_valid.as_str().unwrap();
                        let mut feature_vector: Vec<i64> = vec![0;obj_str_vec.len()];
                        feature_vector.par_iter_mut()
                                      .enumerate()
                                      .for_each(|(i, v)| {
                                        *v = object_wait_time(&config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap(), act1_str, act2_str).num_milliseconds();
                                      });
                        series_vec.push(Series::from_vec(format!("{:?}:{:?}:{:?}", feature, act1_str, act2_str).as_str(), feature_vector));
                    }
                }
            },
            ObjectPoint::ObjectDirectRelationCount => {
                if let Some(f_params) = params {
                    if let Some(relations_valid) = f_params.get("relations") {
                        let rel_enum = Relations::from_str(relations_valid.as_str().unwrap()).unwrap();
                        let mut feature_vector: Vec<u64> = vec![0;obj_str_vec.len()];
                        feature_vector.par_iter_mut()
                                      .enumerate()
                                      .for_each(|(i, v)| {
                                        *v = object_direct_rel_count(&config.ocdg, config.ocdg.object_map.get_by_left(obj_str_vec[i]).unwrap(), &rel_enum) as u64;
                                      });
                        series_vec.push(Series::from_vec(format!("{:?}:{:?}", feature, rel_enum).as_str(), feature_vector));

                    }
                }

            },
            ObjectPoint::ObjectEventsDirectlyFollows => {
                let act_act_order: Vec<(&str, &str)> = config.ocel.activities.iter()
                                                                             .cartesian_product(&config.ocel.activities)
                                                                             .map(|(act1, act2)| {
                                                                                (act1.as_str(), act2.as_str())
                                                                             })
                                                                             .collect();
                let mut feature_vector: Vec<Vec<u64>> = vec![vec![0;act_act_order.len()];obj_str_vec.len()];
                feature_vector.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                let oe_df = object_events_directly_follows(&config.ocel, config.ocel.object_map.get_by_left(obj_str_vec[i]).unwrap());
                                let oe_vec = (0..act_act_order.len()).into_iter()
                                                                     .map(|_| {
                                                                         let curr_pair = act_act_order[i];
                                                                         if let Some(ac1) = oe_df.get(curr_pair.0) {
                                                                             if let Some(ac2) = ac1.get(curr_pair.1) {
                                                                                 return *ac2 as u64;
                                                                             }
                                                                         }
                                                                         0
                                                                     })
                                                                     .collect();
                                *v = oe_vec;
                              });

                for (v, act_act) in transpose(feature_vector).iter().zip(act_act_order) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, act_act).as_str(), v));
                }

            },
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

pub fn activity_existence(log: &Ocel, oid: &usize) -> Vec<u8> {
    let oe_activities: AHashSet<&String> = AHashSet::from_iter(log.objects[&oid].events.iter()
                                            .map(|oe| &log.events[&oe].activity));
    log.activities.iter()
                   .map(|act| {if oe_activities.contains(act) {1} else {0}})
                   .collect_vec()
}


pub fn activity_existence_count(log: &Ocel, oid: &usize) -> Vec<usize> {
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

pub fn activity_value_operator(log: &Ocel, oid: &usize, attr: &str, op: &Operator) -> f64 {
    op.execute(log.objects[&oid].events.iter()
                            .filter(|oe| log.events[&oe].vmap.contains_key(&attr.to_string()))
                            .map(|oe| match &log.events[&oe].vmap[&attr.to_string()] {
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
                        time_diff = log.events[&ev2].timestamp - log.events[&ev1].timestamp;
                        if time_diff < Duration::zero() {
                            time_diff = Duration::zero();
                        }
                    }
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


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::json;

    use crate::objects::{ocel::importer::import_ocel, ocdg::generate_ocdg};

    use super::*;

    static ERROR: f64 = 0.0001;

    lazy_static::lazy_static!{
        static ref OCEL: Ocel = import_ocel("logs/ocel-complex-test.jsonocel").expect("What did you do to the file?");
        static ref OCDG: Ocdg = generate_ocdg(&import_ocel("logs/ocel-complex-test.jsonocel").expect("What did you do to the file?"), &vec![Relations::INTERACTS]);
    }


    #[test]
    fn test_unique_neighbour_count() {
        let oid = OCDG.object_map.get_by_left("r1").expect("cannot fail");
        assert_eq!(unique_neighbor_count(&OCDG, oid), 4); 
    }
    
    #[test]
    fn test_activity_existence() {
        let correct: HashSet<&str> = HashSet::from_iter(["place order", "check availability", "pick item", "receive payment", "send invoice"]);
        let oid = OCEL.object_map.get_by_left("o1").expect("cannot fail");
        assert_eq!(activity_existence(&OCEL, oid).iter().sum::<u8>(), 5);
        activity_existence(&OCEL, oid).iter().enumerate().for_each(|(i, val)| {
            match val {
                0 => {assert!(!correct.contains(&OCEL.activities[i].as_str()))},
                _ => {assert!(correct.contains(&OCEL.activities[i].as_str()))}
            }
        });
    }

    #[test]
    fn test_activity_existence_count() {
        let correct: HashMap<&str, usize> = HashMap::from_iter([("place order", 1), ("check availability", 3), ("pick item", 2), ("receive payment", 1), ("send invoice", 1)]);
        let oid = OCEL.object_map.get_by_left("o1").expect("cannot fail");
        assert_eq!(activity_existence_count(&OCEL, oid).iter().sum::<usize>(), 8); 

        activity_existence_count(&OCEL, oid).iter().enumerate().for_each(|(i, val)| {
            match val {
                0 => {assert!(!correct.contains_key(&OCEL.activities[i].as_str()))},
                _ => {assert_eq!(*val, correct[OCEL.activities[i].as_str()])}
            }
        
        });
    }

    #[test]
    fn test_activity_value_operator() {
        let oid = OCEL.object_map.get_by_left("i1").expect("cannot fail");
        let attr = "prepaid-amount";
        assert_eq!(activity_value_operator(&OCEL, oid, attr, &Operator::Max), 1000.0); 
    }

    #[test]
    fn test_object_lifetime() {
        let oid = OCEL.object_map.get_by_left("i1").expect("cannot fail");
        assert_eq!(object_lifetime(&OCEL, oid).num_milliseconds(), 1980000);
    }

    #[test]
    fn test_unit_set_ratio() {
        let oid = OCEL.object_map.get_by_left("i1").expect("cannot fail");
        assert!((object_unit_set_ratio(&OCEL, oid) - 0.222222).abs() < ERROR);
        let oid = OCEL.object_map.get_by_left("r1").expect("cannot fail");
        assert!((object_unit_set_ratio(&OCEL, oid) - 1.0).abs() < ERROR);
    }
    
    #[test]
    fn test_object_average_event_interaction() {
        let oid = OCEL.object_map.get_by_left("i1").expect("cannot fail");
        assert!((object_average_event_interaction(&OCEL, oid) - 2.44444).abs() < ERROR);
        let oid = OCEL.object_map.get_by_left("r1").expect("cannot fail");
        assert!((object_average_event_interaction(&OCEL, oid) - 1.6).abs() < ERROR);
    }

    #[test]
    fn test_object_type_interaction() {
        let oid = OCDG.object_map.get_by_left("i1").expect("cannot fail");
        assert_eq!(object_type_interaction(&OCDG, oid, "order"), 1);
        let oid = OCDG.object_map.get_by_left("r2").expect("cannot fail");
        assert_eq!(object_type_interaction(&OCDG, oid, "item"), 6);
    }

    #[test]
    fn test_events_directly_follows() {
        let oid = OCEL.object_map.get_by_left("o3").expect("cannot fail");
        let oid_df = object_events_directly_follows(&OCEL, oid);
        assert_eq!(oid_df["place order"]["check availability"], 1);
        assert_eq!(oid_df["pick item"]["send invoice"], 1);
        assert_eq!(oid_df["send invoice"]["receive payment"], 1);
        assert_eq!(oid_df["check availability"]["check availability"], 1);
        assert_eq!(oid_df["check availability"]["pick item"], 1);
        
        // check if double works
        let oid = OCEL.object_map.get_by_left("o1").expect("cannot fail");
        let oid_df = object_events_directly_follows(&OCEL, oid);
        assert_eq!(oid_df["check availability"]["pick item"], 2);
    }

    #[test]
    fn test_object_wait_time() {
        let oid = OCEL.object_map.get_by_left("o1").expect("cannot fail");
        let from_activity = "place order";
        let to_activity = "receive payment";
        assert_eq!(object_wait_time(&OCEL, oid, from_activity, to_activity).num_milliseconds(), 1320000);

    }

    #[test]
    fn test_object_oe_leaf() {
        let oid = OCEL.object_map.get_by_left("o3").expect("cannot fail");
        assert_eq!(object_oe_leaf(&OCEL, oid), true);

        let oid = OCEL.object_map.get_by_left("r1").expect("cannot fail");
        assert_eq!(object_oe_leaf(&OCEL, oid), false);
        
    }

    #[test]
    fn test_object_oe_root() {
        let oid = OCEL.object_map.get_by_left("o3").expect("cannot fail");
        assert_eq!(object_oe_root(&OCEL, oid), true);

        let oid = OCEL.object_map.get_by_left("r1").expect("cannot fail");
        assert_eq!(object_oe_root(&OCEL, oid), false);

    }

    #[test]
    fn test_object_direct_rel_count() {
        let oid = OCDG.object_map.get_by_left("o1").expect("cannot fail");
        assert_eq!(object_direct_rel_count(&OCDG, oid, &Relations::INTERACTS), 2);

        let oid = OCDG.object_map.get_by_left("r2").expect("cannot fail");
        assert_eq!(object_direct_rel_count(&OCDG, oid, &Relations::INTERACTS), 8);
    }

    #[test]
    fn test_user_facing_suite() {
        let mut feature_vec: Vec<(ObjectPoint, Option<Value>)> = vec![];
        feature_vec.push((ObjectPoint::UniqueNeighborCount, None));
        feature_vec.push((ObjectPoint::ObjectWaitTime, Some(json!({"activity_src": "place order", "activity_tar": "receive payment"}))));
        let config = ObjectPointConfig {ocel: &OCEL, ocdg: &OCDG, params: &feature_vec};
        let res = object_point_features(config);
        println!("{}", res);
        // assert_eq!(res[""].sum::<usize>().unwrap(), 9);
        // assert_eq!(res["ActivityOhe:\"place order\":count"].sum::<usize>().unwrap(), 3);
    }
}
