use std::collections::{HashMap, HashSet};

use polars::prelude::Series;
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
    pub fn validate(&self, log: &Ocel, params: &EventSituationParameters, eid: usize) -> bool {
        let event = &log.events[&eid];
        match self {
            EventSituations::EventChoice => {
                match &params.activities {
                    Some(req_activities) => {
                        return req_activities.contains(event.activity.as_str());
                    },
                    None => {}
                }
            },
            EventSituations::EventAttribute => {
                match &params.property {
                    Some(req_property) => {
                        return event.vmap.contains_key(&req_property.to_string());
                    },
                    None => {}
                }
            },
            EventSituations::EventAttributeUnknown => {
                match &params.property {
                    Some(req_property) => {
                        if let Some(val) = event.vmap.get(&req_property.to_string()) {
                            return val.is_null();   
                        }
                    },
                    None => {}
                }
            },
            EventSituations::EventWait => {
                for oid in event.omap.iter() {
                    let obj = &log.objects[&oid];
                    if *obj.events.first().expect("cannot fail") != eid {
                        return true;
                    }
                }
            },
            EventSituations::EventDuration => {},
            EventSituations::EventObjectChoice => {},
            EventSituations::EventMissingRelation => {},
            EventSituations::EventMissingObjectType => {}
        }

        false
    }

    pub fn execute(&self, log: &Ocel, eids: Vec<&usize>) -> Series {
        todo!()
    }
}

#[derive(Default)]
pub struct EventSituationParameters<'a> {
    pub activities: Option<HashSet<&'a str>>,
    pub property: Option<&'a str>,
    pub relations: Option<Vec<&'a Relations>>,
    pub object_types: Option<Vec<&'a str>>
}

pub struct EventSituationConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<EventSituations, EventSituationParameters<'a>>
}


pub fn collect_event_targets(log: &Ocel, situation: EventSituations, params: EventSituationParameters) -> Series {
    let suitable: Vec<&usize> = log.events.keys().filter(|eid| situation.validate(&log, &params, **eid)).collect();
    situation.execute(&log, suitable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ocel::importer::import_ocel;
   
    fn get_test_data() -> Ocel {
        import_ocel("logs/ocel-transformation-test.jsonocel").expect("did you mess with ocel-transformation-test.jsonocel?")
    }

    #[test]
    fn test_event_choice_validate() {
        let log = get_test_data();
        let eid = 0;
        let situation = EventSituations::EventChoice;
        let mut params_good = EventSituationParameters::default();
        params_good.activities = Some(HashSet::from(["A"]));
        let mut params_bad = EventSituationParameters::default();
        params_bad.activities = Some(HashSet::from(["B"]));

        assert!(situation.validate(&log, &params_good, eid));
        assert!(!situation.validate(&log, &params_bad, eid));
    }

    #[test]
    fn test_event_attribute_validate() {
        let log = get_test_data();
        let eid = 0;
        let situation = EventSituations::EventAttribute;
        let mut params_good = EventSituationParameters::default();
        params_good.property = Some("prepaid-amount");
        let mut params_bad = EventSituationParameters::default();
        params_bad.property = Some("not-known-key");

        assert!(situation.validate(&log, &params_good, eid));
        assert!(!situation.validate(&log, &params_bad, eid));
    }

    #[test]
    fn test_event_attribute_unknown_validate() {
        let log = get_test_data();
        let eid = 1;
        let situation = EventSituations::EventAttributeUnknown;
        let mut params_good = EventSituationParameters::default();
        params_good.property = Some("errors");
        let mut params_bad = EventSituationParameters::default();
        params_bad.property = Some("not-known-key");

        assert!(situation.validate(&log, &params_good, eid));
        assert!(!situation.validate(&log, &params_bad, eid));
    }
    
    #[test]
    fn test_event_wait_validate() {
        let log = get_test_data();
        let eid_good = 1;
        let eid_bad = 0;
        let situation = EventSituations::EventWait;
        let params = EventSituationParameters::default();

        assert!(situation.validate(&log, &params, eid_good));
        assert!(!situation.validate(&log, &params, eid_bad));
    }


}
