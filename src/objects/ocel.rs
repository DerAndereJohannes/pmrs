pub mod importer;
pub mod validator;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Ocel {
    #[serde(alias = "ocel:global-log")]
    global_log: HashMap<String, Value>,
    #[serde(alias = "ocel:global-event")]
    global_event: HashMap<String, Value>,
    #[serde(alias = "ocel:global-object")]
    global_object: HashMap<String, Value>,
    #[serde(alias = "ocel:objects")]
    objects: HashMap<String, Value>,
    #[serde(alias = "ocel:events")]
    events: HashMap<String, Value>
}
