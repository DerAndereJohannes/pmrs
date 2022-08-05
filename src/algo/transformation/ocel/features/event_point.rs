use std::collections::HashMap;

use polars::prelude::{DataFrame, Series, NamedFrom};
use rayon::prelude::*;
use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display, IntoEnumIterator};

use crate::objects::{ocel::Ocel, ocdg::{Ocdg, Relations}, linker::link_objects};

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq, Debug)]
pub enum EventPoint {
    RelationCreatedCounts,
    OmapTypeCounts,
    OutputObjectTypeCounts,
    InputObjectTypeCounts,
    ActivityOhe
}


pub struct EventPointConfig<'a> {
    pub ocel: &'a Ocel,
    pub ocdg: &'a Ocdg,
    pub params: &'a Vec<(EventPoint, Option<Value>)>
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


pub fn event_point_features(config: EventPointConfig) -> DataFrame {
    // let object_linker = link_objects(&config.ocel.object_map, &config.ocdg.object_map);
    let ev_str_vec: Vec<&str> = config.ocel.events.keys().map(|eid| config.ocel.event_map.get_by_right(eid).unwrap().as_str()).collect();

    let mut series_vec: Vec<Series> = vec![Series::new("eids", ev_str_vec.clone())];

    for (feature, _params) in config.params {
        match feature {
            EventPoint::RelationCreatedCounts => {
                let mut feature_values: Vec<Vec<u64>> = vec![vec![0;Relations::iter().count()];ev_str_vec.len()];
                feature_values.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                *v = relations_created_counts(&config.ocel, &config.ocdg, &i).iter().map(|rc| *rc as u64).collect();
                              });

                for (v, rel) in transpose(feature_values).iter().zip(Relations::iter()) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, rel).as_str(), v));
                }
            },
            EventPoint::OmapTypeCounts => {
                let ot_order = config.ocel.global_log["ocel:object-types"].as_array().unwrap().to_vec();
                let ot_order_str: Vec<&str> = ot_order.iter().map(|s| s.as_str().unwrap()).collect();
                let mut feature_values: Vec<Vec<u64>> = vec![vec![0;ot_order.len()]; ev_str_vec.len()];
                feature_values.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  let feature = omap_type_counts(&config.ocel, &i);
                                  let map_to_vec = ot_order_str.iter().map(|ot| {
                                                                        match feature.get(*ot) {
                                                                            Some(res) => *res as u64,
                                                                            None => 0 as u64
                                                                        }
                                                                        })
                                                                  .collect::<Vec<u64>>();
                                  *v = map_to_vec;

                              }); 

                for (v, ot) in transpose(feature_values).iter().zip(ot_order_str) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, ot).as_str(), v));
                }
            },
            EventPoint::OutputObjectTypeCounts => {
                let ot_order = config.ocel.global_log["ocel:object-types"].as_array().unwrap().to_vec();
                let ot_order_str: Vec<&str> = ot_order.iter().map(|s| s.as_str().unwrap()).collect();
                let mut feature_values: Vec<Vec<u64>> = vec![vec![0;ot_order.len()]; ev_str_vec.len()];
                feature_values.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  let feature = output_object_type_count(&config.ocel, &i);
                                  let map_to_vec = ot_order_str.iter().map(|ot| {
                                                                        match feature.get(*ot) {
                                                                            Some(res) => *res as u64,
                                                                            None => 0 as u64
                                                                        }
                                                                        })
                                                                  .collect::<Vec<u64>>();
                                  *v = map_to_vec;

                              }); 

                for (v, ot) in transpose(feature_values).iter().zip(ot_order_str) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, ot).as_str(), v));
                }
                
            },
            EventPoint::InputObjectTypeCounts => {
                let ot_order = config.ocel.global_log["ocel:object-types"].as_array().unwrap().to_vec();
                let ot_order_str: Vec<&str> = ot_order.iter().map(|s| s.as_str().unwrap()).collect();
                let mut feature_values: Vec<Vec<u64>> = vec![vec![0;ot_order.len()]; ev_str_vec.len()];
                feature_values.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                  let feature = input_object_type_count(&config.ocel, &i);
                                  let map_to_vec = ot_order_str.iter().map(|ot| {
                                                                        match feature.get(*ot) {
                                                                            Some(res) => *res as u64,
                                                                            None => 0 as u64
                                                                        }
                                                                        })
                                                                  .collect::<Vec<u64>>();
                                  *v = map_to_vec;

                              }); 

                for (v, ot) in transpose(feature_values).iter().zip(ot_order_str) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, ot).as_str(), v));
                }

            },
            EventPoint::ActivityOhe => {
                let mut feature_values: Vec<Vec<u8>> = vec![vec![0;config.ocel.activities.len()];ev_str_vec.len()];
                feature_values.par_iter_mut()
                              .enumerate()
                              .for_each(|(i, v)| {
                                *v = activity_ohe(&config.ocel, &i).iter().map(|rc| *rc as u8).collect();
                              });

                for (v, act) in transpose(feature_values).iter().zip(config.ocel.activities.iter()) {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, act).as_str(), v));
                }
                
            }
        }
    }

    DataFrame::new(series_vec).unwrap()
}


pub fn relations_created_counts(log: &Ocel, ocdg: &Ocdg, eid: &usize) -> Vec<usize> {
    let mut relation_counts: Vec<usize> = vec![0;Relations::iter().count()];
    let linker = link_objects(&log.object_map, &ocdg.object_map);

    if let Some(e) = log.events.get(eid) {
        e.omap.iter()
              .filter(|oid1| ocdg.irels.contains_key(linker.get_by_left(oid1).expect("cannot fail")))
              .for_each(|oid1| {
                  let oid1_convert = linker.get_by_left(oid1).expect("cannot fail");
                  if let Some(relsrc) = ocdg.irels.get(oid1_convert) {
                      relsrc.iter().filter(|(oid2, _)| e.omap.contains(linker.get_by_right(oid2).expect("cannot fail")))
                                   .for_each(|(_oid2, rels)| {
                                       for (rel, rel_events) in rels {
                                           if rel_events.contains(eid) {
                                               relation_counts[*rel as usize] += 1; 
                                           }
                                       }
                                   })
                  }
              })
    }
    relation_counts
}

pub fn omap_type_counts(log:&Ocel, eid: &usize) -> HashMap<String, usize> {
    let mut omap_counts: HashMap<String, usize> = HashMap::new();
    if let Some(e) = log.events.get(eid) {
        e.omap.iter().for_each(|oid| {
            match log.objects.get(oid) {
                Some(obj) => {*omap_counts.entry(obj.obj_type.clone()).or_insert(0) += 1},
                None => {*omap_counts.entry("unknown".to_string()).or_insert(0) += 1}
            }
        });
    }
    omap_counts
}


pub fn output_object_type_count(log:&Ocel, eid: &usize) -> HashMap<String, usize> {
    let mut otype_counts: HashMap<String, usize> = HashMap::new();
    if let Some(e) = log.events.get(eid) {
        e.omap.iter().filter(|oid|{
                                if let Some(obj) = log.objects.get(oid) {
                                    return obj.events.first().expect("cannot fail") == eid;
                                }
                                false
                            })
                     .for_each(|oid| {
                         *otype_counts.entry(log.objects.get(oid).expect("cannot fail").obj_type.clone()).or_insert(0) += 1;
                     });
    }
    otype_counts
} 


pub fn input_object_type_count(log:&Ocel, eid: &usize) -> HashMap<String, usize> {
    let mut otype_counts: HashMap<String, usize> = HashMap::new();
    if let Some(e) = log.events.get(eid) {
        e.omap.iter().filter(|oid|{
                                if let Some(obj) = log.objects.get(oid) {
                                    return obj.events.first().expect("cannot fail") != eid;
                                }
                                false
                            })
                     .for_each(|oid| {
                         *otype_counts.entry(log.objects.get(oid).expect("cannot fail").obj_type.clone()).or_insert(0) += 1;
                     });
    }
    otype_counts
}


pub fn activity_ohe(log: &Ocel, eid: &usize) -> Vec<u8> {
    let mut activity_bools: Vec<u8> = vec![0; log.activities.len()];
    if let Some(e) = log.events.get(eid) {
        if let Some(pos) = log.activities.iter().position(|act| e.activity.as_str() == act) {
            activity_bools[pos] = 1;
        }
    }
    activity_bools
}


#[cfg(test)]
mod tests {
    use crate::objects::{ocel::importer::import_ocel, ocdg::{generate_ocdg, Relations}};

    use super::*;

    lazy_static::lazy_static!{
        static ref OCEL: Ocel = import_ocel("logs/ocel-complex-test.jsonocel").expect("What did you do to the file?");
        static ref OCDG: Ocdg = generate_ocdg(&import_ocel("logs/ocel-complex-test.jsonocel").expect("What did you do to the file?"), &vec![Relations::DESCENDANTS]);
    }

    #[test]
    fn test_relations_creates_counts() {
        assert_eq!(relations_created_counts(&OCEL, &OCDG, &14)[4], 1);
        assert_eq!(relations_created_counts(&OCEL, &OCDG, &32)[4], 2);
        assert_eq!(relations_created_counts(&OCEL, &OCDG, &12)[4], 3);
        assert_eq!(relations_created_counts(&OCEL, &OCDG, &0)[4], 0);
    }

    #[test]
    fn test_omap_type_counts() {
        assert_eq!(omap_type_counts(&OCEL, &0)["order"], 1);
        assert_eq!(omap_type_counts(&OCEL, &0)["item"], 2);

        assert_eq!(omap_type_counts(&OCEL, &33)["route"], 1);
        assert_eq!(omap_type_counts(&OCEL, &33)["item"], 3);
        assert_eq!(omap_type_counts(&OCEL, &33)["package"], 1);
    }

    #[test]
    fn test_output_object_type_counts() {
        assert_eq!(output_object_type_count(&OCEL, &29)["package"], 1);
        assert_eq!(output_object_type_count(&OCEL, &32)["route"], 1);
        assert_eq!(output_object_type_count(&OCEL, &1).is_empty(), true);
    }

    #[test]
    fn test_input_object_type_counts() {
        assert_eq!(input_object_type_count(&OCEL, &0).is_empty(), true);
        assert_eq!(input_object_type_count(&OCEL, &33)["route"], 1);
        assert_eq!(input_object_type_count(&OCEL, &33)["package"], 1);
        assert_eq!(input_object_type_count(&OCEL, &33)["item"], 3);
    }

    #[test]
    fn test_activity_ohe() {
        let correct: HashMap<&str, u8> = HashMap::from_iter([("place order", 1)]);
        
        activity_ohe(&OCEL, &0).iter().enumerate().for_each(|(i, val)| {
            match val {
                0 => {assert!(!correct.contains_key(&OCEL.activities[i].as_str()))},
                _ => {assert_eq!(*val, correct[OCEL.activities[i].as_str()])}
            }
        
        });
        
        let correct: HashMap<&str, u8> = HashMap::from_iter([("failed delivery", 1)]);
        activity_ohe(&OCEL, &18).iter().enumerate().for_each(|(i, val)| {
            match val {
                0 => {assert!(!correct.contains_key(&OCEL.activities[i].as_str()))},
                _ => {assert_eq!(*val, correct[OCEL.activities[i].as_str()])}
            }
        
        });
    }
    
    #[test]
    fn test_user_facing_suite() {
        let mut feature_vec: Vec<(EventPoint, Option<Value>)> = vec![];
        feature_vec.push((EventPoint::RelationCreatedCounts, None));
        feature_vec.push((EventPoint::ActivityOhe, None));
        let config = EventPointConfig {ocel: &OCEL, ocdg: &OCDG, params: &feature_vec};
        let res = event_point_features(config);
        assert_eq!(res["RelationCreatedCounts:DESCENDANTS:count"].sum::<usize>().unwrap(), 9);
        assert_eq!(res["ActivityOhe:\"place order\":count"].sum::<usize>().unwrap(), 3);

    }

}
