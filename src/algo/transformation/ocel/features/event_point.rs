use std::collections::HashMap;

use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display, IntoEnumIterator};

use crate::objects::{ocel::Ocel, ocdg::{Ocdg, Relations}, linker::link_objects};

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

}
