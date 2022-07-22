use std::collections::HashMap;

use chrono::Duration;
use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::{ocel::Ocel, ocdg::Ocdg};

use super::operator::Operator;

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
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
    pub params: &'a HashMap<EventGroup, Option<Value>>

}


pub fn activity_counts(log: &Ocel) -> HashMap<String, usize> {
    let mut activity_counter: HashMap<String, usize> = HashMap::new();
    log.events.iter().for_each(|(_eid, values)| {
                                    *activity_counter.entry(values.activity.clone()).or_insert(0) += 1;
                                });

    activity_counter
}

pub fn activity_attr_operator(log: &Ocel, activity: &str, op: Operator) -> HashMap<String, f64> {
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
                                                          .or_insert(op.execute(values.iter().map(|v| *v)).unwrap());
                  });
    attr_operated
}

pub fn activity_otype_operator(log: &Ocel, op: Operator) -> HashMap<String, HashMap<String, f64>> {
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
              .for_each(|(ot, vals)| {*activity_otype_opmap.entry(act.to_owned()).or_default().entry(ot.to_owned()).or_default() = op.execute(vals.iter().map(|v| *v)).unwrap()})
    });

    activity_otype_opmap
}

pub fn activity_active_time_operator(log: &Ocel, act: &str, op: Operator) -> f64 {
    op.execute(log.events.iter()
              .filter(|(_eid, values)| values.activity == act)
              .map(|(eid, values)| {
                  values.omap.iter()
                             .filter(|oid| log.objects.contains_key(*oid))
                             .map(|oid| {
                                 let pos = log.objects.get(oid).expect("cannot fail").events.iter().position(|v| v == eid).unwrap();
                                 match log.objects.get(oid).expect("cannot fail").events.get(pos+1) {
                                     Some(eid2) => {(log.events[&eid2].timestamp - log.events[&eid].timestamp).num_milliseconds()},
                                     None => {Duration::zero().num_milliseconds()}
                                 }
                             }).max().unwrap() as f64
              })).unwrap()

}

pub fn activity_wait_time_operator(log: &Ocel, act: &str, op: Operator) -> f64 {
    op.execute(log.events.iter()
              .filter(|(_eid, values)| values.activity == act)
              .map(|(eid, values)| {
                  values.omap.iter()
                             .filter(|oid| log.objects.contains_key(*oid))
                             .map(|oid| {
                                 let pos = log.objects.get(oid).expect("cannot fail").events.iter().position(|v| v == eid).unwrap();
                                 match log.objects.get(oid).expect("cannot fail").events.get(pos-1) {
                                     Some(eid2) => {(log.events[&eid].timestamp - log.events[&eid2].timestamp).num_milliseconds()},
                                     None => {Duration::max_value().num_milliseconds()}
                                 }
                             }).min().unwrap() as f64
              })).unwrap()

}

