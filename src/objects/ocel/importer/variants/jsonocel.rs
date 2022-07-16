use crate::objects::ocel::{Ocel, OcelSerde, OcelEvent, OcelObject};
use ahash::AHashSet;
use bimap::BiMap;
use nohash_hasher::{IntMap, IntSet};
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub(crate) fn import_json_ocel(file_path: &str) -> Result<Ocel, Box<dyn Error>> {
    let mut s = String::new();
    File::open(file_path)?.read_to_string(&mut s)?;
    let log: OcelSerde = serde_json::from_str(&s)?;
    let mut log_internal: Ocel = Ocel { global_log: log.global_log, global_event: log.global_event, global_object: log.global_object, events: IntMap::default() , objects: IntMap::default(), object_map: BiMap::new(), event_map: BiMap::new(), activities: AHashSet::new() };

    
    let mut oid_nh: usize = usize::MIN; 
    for (oid, data) in log.objects {
        log_internal.object_map.insert(oid.to_owned(), oid_nh);
        log_internal.objects.insert(oid_nh, OcelObject {oid, obj_type: data.obj_type, ovmap: data.ovmap, events: vec![] });
        oid_nh = oid_nh + 1;
    }

    let mut eid_nh: usize = usize::MIN;
    for (eid, data) in log.events {
        log_internal.activities.insert(data.activity.clone());
        let mut fast_event = OcelEvent {eid: eid.to_owned(), activity: data.activity, timestamp: data.timestamp, vmap: data.vmap, omap: IntSet::default()};
        log_internal.event_map.insert(eid, eid_nh);

        for oid in data.omap.iter() {
            let oid_num = *log_internal.object_map.get_by_left(oid as &str).expect("This was just added so it cannot fail.");
            fast_event.omap.insert(oid_num);
            log_internal.objects.get_mut(&oid_num).unwrap().events.push(eid_nh);
        }

        log_internal.events.insert(eid_nh, fast_event);
        eid_nh = eid_nh + 1;
    }
    
    Ok(log_internal)
}
