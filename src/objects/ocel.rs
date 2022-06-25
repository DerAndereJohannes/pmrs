pub mod importer;
pub mod validator;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use ahash::{AHashMap, AHashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ocel {
    #[serde(alias = "ocel:global-log", rename(serialize = "ocel:global-log"))]
    pub global_log: AHashMap<String, Value>,
    #[serde(alias = "ocel:global-event", rename(serialize = "ocel:global-event"))]
    pub global_event: AHashMap<String, Value>,
    #[serde(alias = "ocel:global-object", rename(serialize = "ocel:global-object"))]
    pub global_object: AHashMap<String, Value>,
    #[serde(alias = "ocel:objects", rename(serialize = "ocel:objects"))]
    pub objects: AHashMap<String, OcelObject>,
    #[serde(alias = "ocel:events", rename(serialize = "ocel:events"))]
    pub events: IndexMap<String, OcelEvent>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelObject{
    #[serde(alias = "ocel:type", rename(serialize = "ocel:type"))]
    pub obj_type: String,
    #[serde(alias = "ocel:ovmap", rename(serialize = "ocel:ovmap"))]
    pub ovmap: AHashMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelEvent {
    #[serde(alias = "ocel:activity", rename(serialize = "ocel:activity"))]
    pub activity: String,
    #[serde(alias = "ocel:timestamp", rename(serialize = "ocel:timestamp"))]
    pub timestamp: DateTime<Utc>,
    #[serde(alias = "ocel:omap", rename(serialize = "ocel:omap"))]
    pub omap: AHashSet<String>,
    #[serde(alias = "ocel:vmap", rename(serialize = "ocel:vmap"))]
    pub vmap: AHashMap<String, Value>

}
