use crate::objects::ocel::{Ocel, OcelSerde, OcelEvent, OcelObject};
use ahash::{AHashMap, AHashSet};
use nohash_hasher::{IntMap, IntSet};
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub(crate) fn import_json_ocel(file_path: &str) -> Result<Ocel, Box<dyn Error>> {
    let mut s = String::new();
    File::open(file_path).unwrap().read_to_string(&mut s).unwrap();
    let log: OcelSerde = serde_json::from_str(&s).unwrap();
    let mut log_internal: Ocel = Ocel { global_log: log.global_log, global_event: log.global_event, global_object: log.global_object, events: IntMap::default() , objects: IntMap::default(), activities: AHashSet::new() };

    let mut oid_nh: usize = usize::MIN; 
    let mut temp_matcher: AHashMap<String, usize> = AHashMap::new();
    for (oid, data) in log.objects {
        temp_matcher.insert(oid.to_owned(), oid_nh);
        log_internal.objects.insert(oid_nh, OcelObject {oid, obj_type: data.obj_type, ovmap: data.ovmap});
        oid_nh = oid_nh + 1;
    }

    let mut eid_nh: usize = usize::MIN;
    for (eid, data) in log.events {
        log_internal.activities.insert(data.activity.clone());
        let mut fast_event = OcelEvent {eid, activity: data.activity, timestamp: data.timestamp, vmap: data.vmap, omap: IntSet::default()};

        for oid in data.omap.iter() {
            fast_event.omap.insert(*temp_matcher.get(oid as &str).unwrap());
        }

        log_internal.events.insert(eid_nh, fast_event);
        eid_nh = eid_nh + 1;
    }
    
    Ok(log_internal)
}
