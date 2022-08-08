pub mod importer;
pub mod exporter;
pub mod validator;

use bimap::BiMap;
use serde::{Serialize, Deserialize, Deserializer};
use nohash_hasher::{IntMap, IntSet};
use serde_json::Value;
use chrono::{DateTime, Utc, TimeZone};
use indexmap::IndexMap;
use ahash::{AHashMap, AHashSet, RandomState};
use std::cmp::Ordering;


#[derive(Serialize, Deserialize, Debug)]
pub struct OcelSerde {
    #[serde(alias = "ocel:global-log", rename(serialize = "ocel:global-log"))]
    pub global_log: AHashMap<String, Value>,
    #[serde(alias = "ocel:global-event", rename(serialize = "ocel:global-event"))]
    pub global_event: AHashMap<String, Value>,
    #[serde(alias = "ocel:global-object", rename(serialize = "ocel:global-object"))]
    pub global_object: AHashMap<String, Value>,
    #[serde(alias = "ocel:objects", rename(serialize = "ocel:objects"))]
    pub objects: AHashMap<String, OcelObjectSerde>,
    #[serde(alias = "ocel:events", rename(serialize = "ocel:events"))]
    pub events: IndexMap<String, OcelEventSerde, RandomState>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OcelObjectSerde{
    #[serde(alias = "ocel:type", rename(serialize = "ocel:type"))]
    pub obj_type: String,
    #[serde(alias = "ocel:ovmap", rename(serialize = "ocel:ovmap"))]
    pub ovmap: AHashMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelEventSerde {
    #[serde(alias = "ocel:activity", rename(serialize = "ocel:activity"))]
    pub activity: String,
    #[serde(alias = "ocel:timestamp", rename(serialize = "ocel:timestamp"), deserialize_with="timezone_default_utc")]
    pub timestamp: DateTime<Utc>,
    #[serde(alias = "ocel:omap", rename(serialize = "ocel:omap"))]
    pub omap: AHashSet<String>,
    #[serde(alias = "ocel:vmap", rename(serialize = "ocel:vmap"))]
    pub vmap: AHashMap<String, Value>,
}

fn timezone_default_utc<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
    let s_input: Option<String> = Deserialize::deserialize(d)?;
    let mut s: String = s_input.unwrap();

    match DateTime::parse_from_rfc3339(s.as_str()) {
        Ok(dt) => {Ok(DateTime::<Utc>::from(dt))},
        Err(_e) => {
            s.push('Z');
            match DateTime::parse_from_rfc3339(s.as_str()) {
                Ok(dt) => {Ok(DateTime::<Utc>::from(dt))},
                Err(_e) => {Ok(DateTime::<Utc>::from(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)))}
            }
        }
    }


}

#[derive(Debug, Clone)]
pub struct Ocel {
    pub global_log: AHashMap<String, Value>,
    pub global_event: AHashMap<String, Value>,
    pub global_object: AHashMap<String, Value>,
    pub events: IntMap<usize, OcelEvent>,
    pub objects: IntMap<usize, OcelObject>,
    pub object_map: BiMap<String, usize>,
    pub event_map: BiMap<String, usize>,
    pub activities: Vec<String>
}


#[derive(Debug, Clone)]
pub struct OcelObject{
    pub obj_type: String,
    pub ovmap: AHashMap<String, Value>,
    pub events: Vec<usize>
}

#[derive(Debug, Eq, Clone)]
pub struct OcelEvent {
    pub activity: String,
    pub timestamp: DateTime<Utc>,
    pub vmap: AHashMap<String, Value>,
    pub omap: IntSet<usize>
}


impl PartialOrd for OcelEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OcelEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialEq for OcelEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}
