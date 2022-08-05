use std::{collections::HashMap, str::FromStr};

use chrono::Duration;
use polars::prelude::{DataFrame, Series, NamedFrom};
use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::{ocel::Ocel, ocdg::Ocdg};

use super::operator::Operator;

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq, Debug)]
pub enum EventGroup {
    ActivityCounts,
    ActivityAttrOperator,
    ActivityObjectTypeOperator,
    ActivityActiveTimeOperator,
    ActivityWaitTimeOperator
}


pub struct EventGroupConfig<'a> {
    pub ocel: &'a Ocel,
    pub ocdg: &'a Ocdg,
    pub params: &'a Vec<(EventGroup, Option<Value>)>

}

pub fn event_group_features(config: EventGroupConfig) -> DataFrame {
    // let object_linker = link_objects(&config.ocel.object_map, &config.ocdg.object_map);
    let mut series_vec: Vec<Series> = vec![];

    for (feature, params) in config.params {
        match feature {
            EventGroup::ActivityCounts => {
                let counts = activity_counts(&config.ocel);
                counts.iter().for_each(|(k, v)| {
                    series_vec.push(Series::new(format!("{:?}:{:?}:count", feature, k).as_str(), vec![*v as u64]));
                });
            },
            EventGroup::ActivityAttrOperator => {
                if let Some(f_params) = params {
                    let op_val: Option<&Value> = f_params.get("operator");
                    let act_val: Option<&Value> = f_params.get("activity");
                    if let (Some(act_valid), Some(op_valid)) = (act_val, op_val) {
                        let act_str = act_valid.as_str().unwrap();
                        let op_enum = Operator::from_str(op_valid.as_str().unwrap()).unwrap();
                        let res = activity_attr_operator(&config.ocel, act_str, &op_enum);

                        res.iter().for_each(|(k, v)| {
                            series_vec.push(Series::new(format!("{:?}:{:?}:{:?}:{:?}", feature, act_str, k, op_enum).as_str(), vec![*v]))
                        });
                    }
                }


            },
            EventGroup::ActivityObjectTypeOperator => {
                if let Some(f_params) = params {
                    let op_val: Option<&Value> = f_params.get("operator");
                    if let Some(op_valid) = op_val {
                        let op_enum = Operator::from_str(op_valid.as_str().unwrap()).unwrap();
                        let res = activity_otype_operator(&config.ocel, &op_enum);

                        res.iter().for_each(|(act, ot_map)| {
                            ot_map.iter().for_each(|(ot, val)| {
                                series_vec.push(Series::new(format!("{:?}:{:?}:{:?}:{:?}", feature, act, ot, op_enum).as_str(), vec![*val]));
                            });
                        });

                    }
                }
                
            },
            EventGroup::ActivityActiveTimeOperator => {
                if let Some(f_params) = params {
                    let op_val: Option<&Value> = f_params.get("operator");
                    let act_val: Option<&Value> = f_params.get("activity");
                    if let (Some(act_valid), Some(op_valid)) = (act_val, op_val) {
                        let act_str = act_valid.as_str().unwrap();
                        let op_enum = Operator::from_str(op_valid.as_str().unwrap()).unwrap();
                        series_vec.push(Series::new(format!("{:?}:{:?}:{:?}", feature, act_str, op_enum).as_str(), vec![activity_active_time_operator(&config.ocel, act_str, &op_enum)]));
                    }
                }
            },
            EventGroup::ActivityWaitTimeOperator => {
                if let Some(f_params) = params {
                    let op_val: Option<&Value> = f_params.get("operator");
                    let act_val: Option<&Value> = f_params.get("activity");
                    if let (Some(act_valid), Some(op_valid)) = (act_val, op_val) {
                        let act_str = act_valid.as_str().unwrap();
                        let op_enum = Operator::from_str(op_valid.as_str().unwrap()).unwrap();
                        series_vec.push(Series::new(format!("{:?}:{:?}:{:?}", feature, act_str, op_enum).as_str(), vec![activity_wait_time_operator(&config.ocel, act_str, &op_enum)]));
                    }
                }

            }
        }
    }


    DataFrame::new(series_vec).unwrap()
}

pub fn activity_counts(log: &Ocel) -> HashMap<String, usize> {
    let mut activity_counter: HashMap<String, usize> = HashMap::new();
    log.events.iter().for_each(|(_eid, values)| {
                                    *activity_counter.entry(values.activity.clone()).or_insert(0) += 1;
                                });

    activity_counter
}

pub fn activity_attr_operator(log: &Ocel, activity: &str, op: &Operator) -> HashMap<String, f64> {
    let mut activity_attrs: HashMap<String, Vec<f64>> = HashMap::new();
    log.events.iter().filter(|(_eid, values)| values.activity == activity)
                     .for_each(|(_eid, values)| {
                         values.vmap.iter().for_each(|(attr, val)| {
                             if let Some(valid) = val.as_f64() {
                                activity_attrs.entry(attr.to_owned()).or_default().push(valid);
                             }
                         });
                     });
    
    let mut attr_operated: HashMap<String, f64> = HashMap::new();

    activity_attrs.iter()
                  .for_each(|(attr, values)| {attr_operated.entry(attr.to_owned())
                                                          .or_insert(op.execute(values.iter().map(|v| *v)).unwrap_or(0.0));
                  });
    attr_operated
}

pub fn activity_otype_operator(log: &Ocel, op: &Operator) -> HashMap<String, HashMap<String, f64>> {
    let mut activity_otype_vecmap: HashMap<String, HashMap<String, Vec<f64>>> = HashMap::new();
    log.events.iter().for_each(|(_eid, values)| {
        let mut omap_counts: HashMap<String, usize> = HashMap::new();
        values.omap.iter().for_each(|oid| {
            match log.objects.get(oid) {
                Some(obj) => {*omap_counts.entry(obj.obj_type.clone()).or_insert(0) += 1},
                None => {*omap_counts.entry("unknown".to_string()).or_insert(0) += 1}
            }
        });
        for (ot, amount) in omap_counts {
            activity_otype_vecmap.entry(values.activity.clone()).or_default().entry(ot).or_default().push(amount as f64);
        }

    });

    let mut activity_otype_opmap: HashMap<String, HashMap<String, f64>> = HashMap::new();

    activity_otype_vecmap.iter().for_each(|(act, othash)| {
        othash.iter()
              .for_each(|(ot, vals)| {*activity_otype_opmap.entry(act.to_owned()).or_default().entry(ot.to_owned()).or_default() = op.execute(vals.iter().map(|v| *v)).unwrap_or(0.0)})
    });

    activity_otype_opmap
}

pub fn activity_active_time_operator(log: &Ocel, act: &str, op: &Operator) -> f64 {
    op.execute(log.events.iter()
              .filter(|(_eid, values)| values.activity == act)
              .map(|(eid, values)| {
                  values.omap.iter()
                             .filter(|oid| log.objects.contains_key(*oid))
                             .map(|oid| {
                                 let pos = log.objects.get(oid).expect("cannot fail").events.iter().position(|v| v == eid).unwrap();
                                 let last_pos = log.objects.get(oid).expect("cannot fail").events.len() - 1;
                                 if pos != last_pos {
                                     let eid2 = log.objects.get(oid).expect("cannot fail").events.get(pos+1).expect("cannot fail");
                                     (log.events[&eid2].timestamp - log.events[&eid].timestamp).num_milliseconds()
                                 } else {
                                     Duration::max_value().num_milliseconds()
                                 }
                             }).max().unwrap() as f64
              })).unwrap_or(0.0)

}

pub fn activity_wait_time_operator(log: &Ocel, act: &str, op: &Operator) -> f64 {
    op.execute(log.events.iter()
              .filter(|(_eid, values)| values.activity == act)
              .map(|(eid, values)| {
                  values.omap.iter()
                             .filter(|oid| log.objects.contains_key(*oid))
                             .map(|oid| {
                                 let pos = log.objects.get(oid).expect("cannot fail").events.iter().position(|v| v == eid).unwrap();
                                 if pos != 0 {
                                     let eid2 = log.objects.get(oid).expect("cannot fail").events.get(pos-1).expect("cannot fail");
                                     (log.events[&eid].timestamp - log.events[&eid2].timestamp).num_milliseconds()
                                 } else {
                                     Duration::max_value().num_milliseconds()
                                 }
                             }).min().unwrap() as f64
              })).unwrap_or(0.0)

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
    fn test_activity_counts() {
        let ac_counts = activity_counts(&OCEL);
        assert_eq!(ac_counts["place order"], 3);
        assert_eq!(ac_counts["check availability"], 9);
        assert_eq!(ac_counts["failed delivery"], 1);
    }

    #[test]
    fn test_activity_attr_operator() {
        let ac_attr = activity_attr_operator(&OCEL, "place order", &Operator::Max);
        assert_eq!(ac_attr["prepaid-amount"], 1000.0);
        
        let ac_attr = activity_attr_operator(&OCEL, "check availability", &Operator::Min);
        assert_eq!(ac_attr["time-taken"], 2.0);
        assert_eq!(ac_attr["effort"], 2.0);

    }

    #[test]
    fn test_activity_otype_operator() {
        let ac_otype = activity_otype_operator(&OCEL, &Operator::Max);
        assert_eq!(ac_otype["check availability"]["item"], 1.0);
        assert_eq!(ac_otype["store package"]["item"], 3.0);
        assert_eq!(ac_otype["unload package"]["route"], 1.0);
    }

    #[test]
    fn test_activity_active_time_operator() {
        assert_eq!(activity_active_time_operator(&OCEL, "place order", &Operator::Mean), 280000.0);
        assert_eq!(activity_active_time_operator(&OCEL, "start route", &Operator::Mean), 90000.0);
        assert_eq!(activity_active_time_operator(&OCEL, "receive payment", &Operator::Max), Duration::max_value().num_milliseconds() as f64);
    }

    #[test]
    fn test_activity_wait_time_operator() {
        assert_eq!(activity_wait_time_operator(&OCEL, "receive payment", &Operator::Max), 360000.0);
        assert_eq!(activity_wait_time_operator(&OCEL, "place order", &Operator::Max), Duration::max_value().num_milliseconds() as f64);
    }

    #[test]
    fn test_user_facing_suite() {
        let mut feature_vec: Vec<(EventGroup, Option<Value>)> = vec![];
        feature_vec.push((EventGroup::ActivityCounts, None));
        let config = EventGroupConfig {ocel: &OCEL, ocdg: &OCDG, params: &feature_vec};
        let res = event_group_features(config);
        assert_eq!(res["ActivityCounts:\"pick item\":count"].sum::<i8>().unwrap(), 6);
    }
}
