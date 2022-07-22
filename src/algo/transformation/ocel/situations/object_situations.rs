use std::collections::HashMap;

use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::ocel::Ocel;


#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum ObjectSituations {
    ObjectAttribute,
    ObjectAttributeUnknown,
    ObjectMissingActivity,
    ObjectLifetime,
    ObjectMissingReachableObjectType
}


pub struct ObjectSituationConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<ObjectSituations, Option<Value>>
}

