use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::{ocel::Ocel, ocdg::Relations};




#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum EventSituations {
    EventChoice,
    EventAttribute,
    EventAttributeUnknown,
    EventWait,
    EventDuration,
    EventObjectChoice,
    EventMissingRelation,
    EventMissingObjectType
}

impl EventSituations {
    pub fn execute(&self, log: &Ocel, params: &EventSituationParameters, eid: usize) -> Option<Value> {
        let event = &log.events[&eid];
        match self {
            EventSituations::EventChoice => {
                if let Some(req_activities) = &params.activities {
                    if req_activities.contains(event.activity.as_str()) {
                        return Some(json!(event.activity));
                    }
                }
            },
            EventSituations::EventAttribute => {
                if let Some(req_property) = &params.property {
                    if let Some(val) = event.vmap.get(&req_property.to_string()) {
                        if !val.is_null() {
                            return Some(val.to_owned());
                        }
                    }
                }
            },
            EventSituations::EventAttributeUnknown => {
                if let Some(req_property) = &params.property {
                    if let Some(val) = event.vmap.get(&req_property.to_string()) {
                        return Some(val.to_owned());
                    }
                }
            },
            EventSituations::EventWait => {
                let mut oldest_time: &DateTime<Utc> = &chrono::MAX_DATETIME;
                for oid in event.omap.iter() {
                    let obj = &log.objects[&oid];
                    if *obj.events.first().expect("cannot fail") != eid {
                        let prev_event: &usize = &obj.events[obj.events.iter().position(|&eid2| eid2 == eid).unwrap() - 1];
                        let prev_timestamp: &DateTime<Utc> = &log.events[prev_event].timestamp;
                        if prev_timestamp < oldest_time {
                            oldest_time = prev_timestamp;
                        }
                    }
                }

                if oldest_time != &chrono::MAX_DATETIME {
                    return Some(json!(log.events[&eid].timestamp.timestamp_millis() - oldest_time.timestamp_millis()));
                }
            },
            EventSituations::EventDuration => {
                let mut youngest_time: &DateTime<Utc> = &chrono::MIN_DATETIME;
                for oid in event.omap.iter() {
                    let obj = &log.objects[&oid];
                    if *obj.events.last().expect("cannot fail") != eid {
                        let prev_event: &usize = &obj.events[obj.events.iter().position(|&eid2| eid2 == eid).unwrap() + 1];
                        let next_timestamp: &DateTime<Utc> = &log.events[prev_event].timestamp;
                        if next_timestamp > youngest_time {
                            youngest_time = next_timestamp;
                        }
                    }
                }

                if youngest_time != &chrono::MIN_DATETIME {
                    return Some(json!(youngest_time.timestamp_millis() - log.events[&eid].timestamp.timestamp_millis()));
                }
            },
            EventSituations::EventObjectChoice => {
                if let Some(acts) = &params.activities { 
                    if acts.contains(event.activity.as_str()) {
                        if let Some(otypes) = &params.object_types {
                            let mut oid_fit: Vec<&usize> = vec![];
                            for oid in event.omap.iter() {
                                if otypes.contains(&log.objects[&oid].obj_type.as_str()) {
                                    oid_fit.push(oid);
                                }
                            }
                            if !oid_fit.is_empty() {
                                return Some(json!(oid_fit));
                            }
                        }
                    }
                }
            },
            EventSituations::EventMissingObjectType => {
                if let Some(otypes) = &params.object_types {
                    let omap_otypes: HashSet<&str> = HashSet::from_iter(event.omap.iter().map(|o| log.objects[o].obj_type.as_str()));
                    let mut ot_fit = vec![];
                    for ot in otypes {
                        if !omap_otypes.contains(ot) {
                            ot_fit.push(ot);
                        }
                    }

                    if !ot_fit.is_empty() {
                        return Some(json!(ot_fit));
                    }
                }

            },
            EventSituations::EventMissingRelation => {}
        }
        None
    }
}

#[derive(Default)]
pub struct EventSituationParameters<'a> {
    pub activities: Option<HashSet<&'a str>>,
    pub property: Option<&'a str>,
    pub relations: Option<Vec<&'a Relations>>,
    pub object_types: Option<HashSet<&'a str>>
}

pub struct EventSituationConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<EventSituations, EventSituationParameters<'a>>
}


pub fn collect_event_targets(log: &Ocel, situation: EventSituations, params: EventSituationParameters) -> Vec<(usize, Value)> {
    log.events.keys()
              .map(|eid| (*eid, situation.execute(&log, &params, *eid)))
              .filter(|(_, val)| val.is_some())
              .map(|(eid, val)| (eid, val.unwrap()))
              .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ocel::importer::import_ocel;
   
    fn get_test_data() -> Ocel {
        import_ocel("logs/ocel-transformation-test.jsonocel").expect("did you mess with ocel-transformation-test.jsonocel?")
    }

    #[test]
    fn test_event_choice_execute() {
        let log = get_test_data();
        let eid = 0;
        let situation = EventSituations::EventChoice;
        let mut params_good = EventSituationParameters::default();
        params_good.activities = Some(HashSet::from(["A"]));
        let mut params_bad = EventSituationParameters::default();
        params_bad.activities = Some(HashSet::from(["B"]));

        assert_eq!(situation.execute(&log, &params_good, eid).unwrap(), json!("A"));
        assert_eq!(situation.execute(&log, &params_bad, eid), None);
    }

    #[test]
    fn test_event_attribute_execute() {
        let log = get_test_data();
        let eid = 0;
        let situation = EventSituations::EventAttribute;
        let mut params_good = EventSituationParameters::default();
        params_good.property = Some("prepaid-amount");
        let mut params_bad = EventSituationParameters::default();
        params_bad.property = Some("not-known-key");

        assert_eq!(situation.execute(&log, &params_good, eid).unwrap(), json!(1000.0));
        assert_eq!(situation.execute(&log, &params_bad, eid), None);
    }

    #[test]
    fn test_event_attribute_unknown_execute() {
        let log = get_test_data();
        let eid = 1;
        let situation = EventSituations::EventAttributeUnknown;
        let mut params_good = EventSituationParameters::default();
        params_good.property = Some("errors");
        let mut params_bad = EventSituationParameters::default();
        params_bad.property = Some("not-known-key");

        assert_eq!(situation.execute(&log, &params_good, eid).unwrap(), json!(null));
        assert_eq!(situation.execute(&log, &params_bad, eid), None);
    }
    
    #[test]
    fn test_event_wait_execute() {
        let log = get_test_data();
        let eid_good = 1;
        let eid_bad = 0;
        let situation = EventSituations::EventWait;
        let params = EventSituationParameters::default();

        assert_eq!(situation.execute(&log, &params, eid_good).unwrap(), json!(3600000));
        assert_eq!(situation.execute(&log, &params, eid_bad), None);
    }

    #[test]
    fn test_event_duration_execute() {
        let log = get_test_data();
        let eid_good = 1;
        let eid_bad = 2;
        let situation = EventSituations::EventDuration;
        let params = EventSituationParameters::default();

        assert_eq!(situation.execute(&log, &params, eid_good).unwrap(), json!(3600000));
        assert_eq!(situation.execute(&log, &params, eid_bad), None);
    }


    #[test]
    fn test_event_object_choice_execute() {
        let log = get_test_data();
        let eid = 0;
        let situation = EventSituations::EventObjectChoice;
        let mut params = EventSituationParameters::default();
        params.activities = Some(HashSet::from(["A"]));
        params.object_types = Some(HashSet::from(["order"]));

        let o1oid: &usize = log.object_map.get_by_left(&"o1".to_string()).unwrap();

        assert_eq!(situation.execute(&log, &params, eid).unwrap(), json!(vec![o1oid]));

        params.object_types = Some(HashSet::from(["worker"]));
        assert_eq!(situation.execute(&log, &params, eid), None);
    }

    #[test]
    fn test_event_missing_object_type_execute() {
        let log = get_test_data();
        let eid = 0;
        let situation = EventSituations::EventMissingObjectType;
        let mut params = EventSituationParameters::default();
        params.object_types = Some(HashSet::from(["worker"]));

        assert_eq!(situation.execute(&log, &params, eid).unwrap(), json!(vec!["worker"]));

        params.object_types = Some(HashSet::from(["order"]));
        assert_eq!(situation.execute(&log, &params, eid), None);
    }

    #[test]
    fn test_collect_event_targets() {
        let log = get_test_data();
        let mut params = EventSituationParameters::default();
        params.activities = Some(HashSet::from(["A"]));
        let situation = EventSituations::EventChoice;

        let test_execution = collect_event_targets(&log, situation, params);
        assert!(test_execution.len() == 1);
        assert!(test_execution.first().unwrap().to_owned() == (0, json!("A")));
    }

}
