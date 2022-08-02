use std::collections::{HashMap, HashSet};

use serde_json::{Value, json};
use strum::{EnumString, IntoStaticStr, Display};
use crate::objects::ocel::Ocel;

lazy_static::lazy_static!{
static ref NULLVALUES: Vec<Value> = vec![json!("NaN"), json!("NA"), json!("na"), json!("n/a")];
}

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum ObjectSituations {
    ObjectAttribute,
    ObjectAttributeUnknown,
    ObjectMissingActivity,
    ObjectLifetime,
    ObjectMissingReachableObjectType
}


impl ObjectSituations {
    pub fn execute(&self, log: &Ocel, params: &ObjectSituationParameters, oid: usize) -> Option<Value> {
        let obj = &log.objects[&oid];
        match self {
            ObjectSituations::ObjectAttribute => {
                if let Some(req_property) = &params.property {
                    if obj.ovmap.contains_key(&req_property.to_string()) {
                        return Some(obj.ovmap[&req_property.to_string()].to_owned());
                    }
                }
            },
            ObjectSituations::ObjectAttributeUnknown => {
                if let Some(req_property) = &params.property {
                    if let Some(val) = obj.ovmap.get(&req_property.to_string()) {
                        if val.is_null() || NULLVALUES.contains(&val) {
                            return Some(val.to_owned());
                        }
                    }
               } 
            },
            ObjectSituations::ObjectMissingActivity => {
                if let Some(activities) = &params.activities {
                    let oe_activities: HashSet<&str> = HashSet::from_iter(obj.events.iter().map(|eid| log.events[eid].activity.as_str()));
                    let mut remaining: Vec<&str> = vec![];
                    for ac in activities {
                        if !oe_activities.contains(ac) {
                            remaining.push(ac);
                        }
                    }
                    if !remaining.is_empty() {
                        return Some(json!(remaining));
                    }
                }
            },
            ObjectSituations::ObjectLifetime => {
                if let Some(activities) = &params.activities {
                    let final_eid = obj.events.last().expect("this cannot fail");
                    if activities.contains(log.events[&final_eid].activity.as_str()) {
                        let first_ev = log.events[obj.events.first().expect("this cannot fail")].timestamp;
                        let last_ev = log.events[final_eid].timestamp;
                        return Some(json!(last_ev.timestamp_millis() - first_ev.timestamp_millis()));
                    }
                } 
            },
            ObjectSituations::ObjectMissingReachableObjectType => {todo!()}
        }
        None
    }
}

#[derive(Default)]
pub struct ObjectSituationParameters<'a> {
    pub activities: Option<HashSet<&'a str>>,
    pub property: Option<&'a str>,
    pub object_types: Option<HashSet<&'a str>>
}


pub struct ObjectSituationConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<ObjectSituations, ObjectSituationParameters<'a>>
}

pub fn collect_object_targets(log: &Ocel, situation: ObjectSituations, params: ObjectSituationParameters) -> Vec<(usize, Value)> {
    log.objects.keys()
              .map(|oid| (*oid, situation.execute(&log, &params, *oid)))
              .filter(|(_, val)| val.is_some())
              .map(|(oid, val)| (oid, val.unwrap()))
              .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::objects::ocel::importer::import_ocel;
   
    fn get_test_data() -> Ocel {
        import_ocel("logs/ocel-transformation-test.jsonocel").expect("did you mess with ocel-transformation-test.jsonocel?")
    }

    #[test]
    fn test_object_attribute_validation() {
        let log = get_test_data();
        let oid = log.object_map.get_by_left(&"o1".to_string()).unwrap().to_owned();
        let situation = ObjectSituations::ObjectAttribute;
        let mut params_good = ObjectSituationParameters::default();
        params_good.property = Some("total");
        let mut params_bad = ObjectSituationParameters::default();
        params_bad.property = Some("notaproperty");

        assert_eq!(situation.execute(&log, &params_good, oid).unwrap(), json!(2999.99));
        assert_eq!(situation.execute(&log, &params_bad, oid), None);
    }

    #[test]
    fn test_object_attribute_unknown_validation() {
        let log = get_test_data();
        let oid = log.object_map.get_by_left(&"i1".to_string()).unwrap().to_owned();
        let situation = ObjectSituations::ObjectAttributeUnknown;
        let mut params_good = ObjectSituationParameters::default();
        params_good.property = Some("name");
        let mut params_bad = ObjectSituationParameters::default();
        params_bad.property = Some("price");

        assert_eq!(situation.execute(&log, &params_good, oid).unwrap(), json!(null));
        assert_eq!(situation.execute(&log, &params_bad, oid), None);
    }
        

    #[test]
    fn test_object_missing_activity_validation() {
        let log = get_test_data();
        let oid = log.object_map.get_by_left(&"o1".to_string()).unwrap().to_owned();
        let situation = ObjectSituations::ObjectMissingActivity;
        let mut params_good = ObjectSituationParameters::default();
        params_good.activities = Some(HashSet::from(["B"]));
        let mut params_bad = ObjectSituationParameters::default();
        params_bad.activities = Some(HashSet::from(["A"]));

        assert_eq!(situation.execute(&log, &params_good, oid).unwrap(), json!(vec!["B"]));
        assert_eq!(situation.execute(&log, &params_bad, oid), None);
    } 

    #[test]
    fn test_object_lifetime_validation() {
        let log = get_test_data();
        let oid_good = log.object_map.get_by_left(&"i1".to_string()).unwrap().to_owned();
        let oid_bad = log.object_map.get_by_left(&"o1".to_string()).unwrap().to_owned();
        let situation = ObjectSituations::ObjectLifetime;
        let mut params = ObjectSituationParameters::default();
        params.activities = Some(HashSet::from(["B"]));

        assert_eq!(situation.execute(&log, &params, oid_good).unwrap(), json!(3600000));
        assert_eq!(situation.execute(&log, &params, oid_bad), None);
    } 
    
    #[test]
    fn test_collect_object_targets() {
        let log = get_test_data();
        let oid = log.object_map.get_by_left(&"o1".to_string()).unwrap().to_owned();
        let mut params = ObjectSituationParameters::default();
        params.activities = Some(HashSet::from(["B"]));
        let situation = ObjectSituations::ObjectMissingActivity;

        let test_execution = collect_object_targets(&log, situation, params);
        assert_eq!(test_execution.len(), 1);
        assert_eq!(test_execution.first().unwrap().to_owned(), (oid, json!(["B"])));
    }
}
