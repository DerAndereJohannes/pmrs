pub mod importer;
pub mod validator;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ocel {
    #[serde(alias = "ocel:global-log", rename(serialize = "ocel:global-log"))]
    pub global_log: HashMap<String, Value>,
    #[serde(alias = "ocel:global-event", rename(serialize = "ocel:global-event"))]
    pub global_event: HashMap<String, Value>,
    #[serde(alias = "ocel:global-object", rename(serialize = "ocel:global-object"))]
    pub global_object: HashMap<String, Value>,
    #[serde(alias = "ocel:objects", rename(serialize = "ocel:objects"))]
    pub objects: HashMap<String, OcelObject>,
    #[serde(alias = "ocel:events", rename(serialize = "ocel:events"))]
    pub events: IndexMap<String, OcelEvent>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelObject{
    #[serde(alias = "ocel:type", rename(serialize = "ocel:type"))]
    pub obj_type: String,
    #[serde(alias = "ocel:ovmap", rename(serialize = "ocel:ovmap"))]
    pub ovmap: HashMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelEvent {
    #[serde(alias = "ocel:activity", rename(serialize = "ocel:activity"))]
    pub activity: String,
    #[serde(alias = "ocel:timestamp", rename(serialize = "ocel:timestamp"))]
    pub timestamp: DateTime<Utc>,
    #[serde(alias = "ocel:omap", rename(serialize = "ocel:omap"))]
    pub omap: HashSet<String>,
    #[serde(alias = "ocel:vmap", rename(serialize = "ocel:vmap"))]
    pub vmap: HashMap<String, Value>

}
