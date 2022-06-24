pub mod importer;
pub mod validator;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ocel {
    #[serde(alias = "ocel:global-log")]
    pub global_log: HashMap<String, Value>,
    #[serde(alias = "ocel:global-event")]
    pub global_event: HashMap<String, Value>,
    #[serde(alias = "ocel:global-object")]
    pub global_object: HashMap<String, Value>,
    #[serde(alias = "ocel:objects")]
    pub objects: HashMap<String, OcelObject>,
    #[serde(alias = "ocel:events")]
    pub events: IndexMap<String, OcelEvent>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelObject{
    #[serde(alias = "ocel:type")]
    pub obj_type: String,
    #[serde(alias = "ocel:ovmap")]
    pub ovmap: HashMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OcelEvent {
    #[serde(alias = "ocel:activity")]
    pub activity: String,
    #[serde(alias = "ocel:timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(alias = "ocel:omap")]
    pub omap: HashSet<String>,
    #[serde(alias = "ocel:vmap")]
    pub vmap: HashMap<String, Value>

}
