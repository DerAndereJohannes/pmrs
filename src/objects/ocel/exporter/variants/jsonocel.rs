#[allow(unused_imports)]
use crate::objects::ocel::{Ocel, OcelSerde, OcelEvent, OcelObject, OcelEventSerde, OcelObjectSerde};
use ahash::{AHashMap, AHashSet};
use indexmap::IndexMap;
use std::{fs::OpenOptions, io::{BufWriter, Write}, error::Error};

pub(crate) fn export_json_ocel(log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log_serde: OcelSerde = generate_ocel_serde(log);

    let serde_ocel = serde_json::to_string(&log_serde).unwrap();
    let output_file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path).unwrap();
    
    let mut f = BufWriter::new(output_file);
    f.write_all(serde_ocel.as_bytes()).expect("Unable to write data");

    Ok(true)
}

pub(crate) fn export_json_ocel_pretty(log: &Ocel, file_path: &str) -> Result<bool, Box<dyn Error>> {
    let log_serde: OcelSerde = generate_ocel_serde(log);

    let serde_ocel = serde_json::to_string_pretty(&log_serde).unwrap();
    let output_file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path).unwrap();
    
    let mut f = BufWriter::new(output_file);
    f.write_all(serde_ocel.as_bytes()).expect("Unable to write data");

    Ok(true)
}


pub(self) fn generate_ocel_serde(log: &Ocel) -> OcelSerde {
    let mut log_serde: OcelSerde = OcelSerde { global_log: log.global_log.to_owned(), global_event: log.global_event.to_owned(), global_object: log.global_object.to_owned(), objects: AHashMap::new(), events: IndexMap::new() };

    let mut temp_matcher: AHashMap<usize, String> = AHashMap::new();

    for (oid, data) in &log.objects {
        temp_matcher.insert(*oid, data.oid.to_owned()); 
        log_serde.objects.insert(data.oid.to_owned(), OcelObjectSerde { obj_type: data.obj_type.to_owned(), ovmap: data.ovmap.to_owned() });
    }


    for (_, data) in &log.events {
        let mut serde_event = OcelEventSerde {activity: data.activity.to_owned(), timestamp: data.timestamp, vmap: data.vmap.to_owned(), omap: AHashSet::new()};

        for oid in data.omap.iter() {
            serde_event.omap.insert(temp_matcher.get(oid).unwrap().to_string());
        }
        log_serde.events.insert(data.eid.to_owned(), serde_event);
    }
    
    log_serde

}
