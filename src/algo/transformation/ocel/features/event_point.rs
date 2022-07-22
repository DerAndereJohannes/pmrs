use std::collections::HashMap;

use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display, IntoEnumIterator};

use crate::objects::{ocel::Ocel, ocdg::{Ocdg, Relations}};

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
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
    pub params: &'a HashMap<EventPoint, Option<Value>>
}


pub fn relations_created_counts(log: &Ocel, ocdg: &Ocdg, eid: &usize) -> Vec<usize> {
    let mut relation_counts: Vec<usize> = vec![0;Relations::iter().count()];
    if let Some(e) = log.events.get(eid) {
        e.omap.iter()
              .filter(|oid1| ocdg.irels.contains_key(oid1))
              .for_each(|oid1| {
                  let relsrc = ocdg.irels.get(oid1).unwrap();
                  relsrc.iter().filter(|(oid2, _)| e.omap.contains(oid2))
                               .for_each(|(_oid2, rels)| {
                                   for (rel, rel_events) in rels {
                                       if rel_events.contains(eid) {
                                           relation_counts[*rel as usize] += 1; 
                                       }
                                   }
                               })
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
