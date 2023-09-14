use ahash::AHashMap;
use chrono::Utc;
use chrono::DateTime;
use nohash_hasher::IntSet;
use serde_json::Value;
use crate::objects::ocel::Ocel;

use super::OcelEvent;
use super::OcelObject;

struct BuilderObject<'a>(&'a str, &'a str);


trait LogBuilder {
    fn add_event(&mut self, name: &str, time: DateTime<Utc>, activity: &str, obj: Vec<BuilderObject>, properties: Option<AHashMap<String, Value>>);
}


impl LogBuilder for Ocel {
    fn add_event(&mut self, event_name: &str, time: DateTime<Utc>, activity: &str, objs: Vec<BuilderObject>, properties: Option<AHashMap<String, Value>>) {
        // create and gather objects
        let omap_ids: IntSet<usize> = objs.iter().map(|bo| {
            match self.object_map.get_by_left(bo.1) {
                Some(id) => {id.to_owned()},
                None => {
                    let new_id = self.objects.len();
                    let new_object: OcelObject = OcelObject { obj_type: bo.0.to_string(), ovmap: AHashMap::new(), events: vec![] };
                    self.objects.insert(new_id, new_object);
                    self.object_map.insert(bo.1.to_string(), new_id);
                    new_id
                }
            }
        }).collect();

        // Generate event details
        let new_ev_id = self.events.len();
        
        let ev_properties = match properties {
            Some(prop) => prop,
            None => AHashMap::default()
        };

        let new_event = OcelEvent {activity: activity.to_string(), timestamp: time, omap: omap_ids, vmap: ev_properties};
        self.event_map.insert(event_name.to_string(), new_ev_id);
        self.events.insert(new_ev_id, new_event);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_event_add() {
        let mut log = Ocel::default();

        log.add_event("e0", Utc::now(), "order items", vec![BuilderObject("order", "o1"), BuilderObject("item", "i1")], None);

        assert_eq!(log.events.len(), 1);
        assert_eq!(log.event_map.get_by_left("e0").expect("The event was not added to the event_map"), &0);
        
        match log.events.get(&0) {
            Some(event) => {
                assert_eq!(event.activity, "order items".to_string());
                assert_eq!(event.omap.len(), 2);
                assert_eq!(event.vmap.len(), 0);
            },
            None => {
                panic!("The event was not added correctly at index 0");
            }
        }

        // add another event but with already containing items
        log.add_event("e1", Utc::now(), "receive items", vec![BuilderObject("order", "o1"), BuilderObject("item", "i1")], None);

        assert_eq!(log.objects.len(), 2);
        assert_eq!(log.events.len(), 2);
        assert_eq!(log.object_map.len(), 2);
        assert_eq!(log.event_map.len(), 2);

    }

}
