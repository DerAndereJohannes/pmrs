use std::collections::HashMap;

use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::ocel::Ocel;


#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum EventSituations {
    EventChoice,
    EventAttribute,
    EventAttributeUnknown,
    EventWait,
    EventDuration,
    EventObjectChoice,
    EventMissingRelation,
    EventMissingObjectType
}


pub struct EventSituationConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<EventSituations, Option<Value>>
}

