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
    fn add_event(&mut self, name: &str, time: DateTime<Utc>, activity: &str, obj: Vec<BuilderObject>, properties: AHashMap<String, Value>);
}


impl LogBuilder for Ocel {
    fn add_event(&mut self, event_name: &str, time: DateTime<Utc>, activity: &str, objs: Vec<BuilderObject>, properties: AHashMap<String, Value>) {
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
        let new_event = OcelEvent {activity: activity.to_string(), timestamp: time, omap: omap_ids, vmap: properties};
        self.event_map.insert(event_name.to_string(), new_ev_id);
        self.events.insert(new_ev_id, new_event);
    }
}
