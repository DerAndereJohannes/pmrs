use std::iter::FromIterator;

use nohash_hasher::{IntMap, IntSet};
use strum::EnumString;

use crate::objects::ocel::{Ocel, OcelEvent, OcelObject};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString)]
pub enum ExtractionPlan {
    PrefixRoot,
    RootSuffix,
    AllEvents
}

pub fn generate_event_situation_sublog(log: &Ocel, extraction_plan: ExtractionPlan, eid: &usize) -> Option<Ocel> {
    let event_obj = log.events.get(eid)?;
    let mut event_set: IntSet<usize> = IntSet::default();

    let mut object_set: IntSet<usize> = IntSet::default();
    object_set.extend(&event_obj.omap);

    for oid in &event_obj.omap {
        if let Some(object_obj) = log.objects.get(oid) {
            for oe_eid in &object_obj.events {
                match extraction_plan {
                    ExtractionPlan::PrefixRoot => {
                        if oe_eid <= eid {
                            if let Some(oe_event_obj) = log.events.get(oe_eid) {
                                event_set.insert(*oe_eid);
                                object_set.extend(&oe_event_obj.omap);
                            }
                        }
                    },
                    ExtractionPlan::RootSuffix => {
                        if oe_eid >= eid {
                            if let Some(oe_event_obj) = log.events.get(oe_eid) {
                                event_set.insert(*oe_eid);
                                object_set.extend(&oe_event_obj.omap);
                            }
                        }
                    },
                    ExtractionPlan::AllEvents => {
                        if let Some(oe_event_obj) = log.events.get(oe_eid) {
                            event_set.insert(*oe_eid);
                            object_set.extend(&oe_event_obj.omap);
                        }
                    }
                } 
            }
        }
    }

    let events: IntMap<usize, OcelEvent> = IntMap::from_iter(event_set.into_iter().map(|ev| (ev, log.events[&ev].clone())));
    let objects: IntMap<usize, OcelObject> = IntMap::from_iter(object_set.into_iter().map(|obj| (obj, log.objects[&obj].clone())));

    Some(Ocel {activities: log.activities.clone(), event_map: log.event_map.clone(), object_map: log.object_map.clone(), global_log: log.global_log.clone(), global_event: log.global_event.clone(), global_object: log.global_object.clone(), events, objects })

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ocel::importer::import_ocel;

    lazy_static::lazy_static!{
        static ref OCEL: Ocel = import_ocel("logs/ocel-complex-test.jsonocel").expect("What did you do to the file?");
    }

    #[test]
    fn test_event_situation_sublog_prefix_root() {
        let test_eid = OCEL.event_map.get_by_left("e36").expect("test log was altered");
        let sublog = generate_event_situation_sublog(&OCEL, ExtractionPlan::PrefixRoot, &test_eid);
        
        assert!(sublog.is_some());
        if let Some(sublog_unwrapped) = sublog {
            assert_eq!(sublog_unwrapped.events.len(), 6);
            assert_eq!(*sublog_unwrapped.events.keys().max().unwrap(), 35);
            assert_eq!(*sublog_unwrapped.events.keys().min().unwrap(), 29);
        }
    }

    #[test]
    fn test_event_situation_sublog_root_suffix() {
        let test_eid = OCEL.event_map.get_by_left("e5").expect("test log was altered");
        let sublog = generate_event_situation_sublog(&OCEL, ExtractionPlan::RootSuffix, &test_eid);
        
        assert!(sublog.is_some());
        if let Some(sublog_unwrapped) = sublog {
            assert_eq!(sublog_unwrapped.events.len(), 9);
            assert_eq!(*sublog_unwrapped.events.keys().max().unwrap(), 33);
            assert_eq!(*sublog_unwrapped.events.keys().min().unwrap(), 4);
        }
    }

    #[test]
    fn test_event_situation_sublog_all_events() {
        let test_eid = OCEL.event_map.get_by_left("e5").expect("test log was altered");
        let sublog = generate_event_situation_sublog(&OCEL, ExtractionPlan::AllEvents, &test_eid);
        
        assert!(sublog.is_some());
        if let Some(sublog_unwrapped) = sublog {
            assert_eq!(sublog_unwrapped.events.len(), 12);
            assert_eq!(*sublog_unwrapped.events.keys().max().unwrap(), 33);
            assert_eq!(*sublog_unwrapped.events.keys().min().unwrap(), 0);
        }
    }

    #[test]
    fn test_event_situation_invalid_eid_success() {
        let test_eid = 99; // does not exist
        let sublog = generate_event_situation_sublog(&OCEL, ExtractionPlan::AllEvents, &test_eid);
        assert!(sublog.is_none());
    }
}
