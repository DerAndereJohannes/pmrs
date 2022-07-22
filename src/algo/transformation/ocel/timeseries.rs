use std::collections::HashMap;

use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};

use crate::objects::ocel::Ocel;


#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum TimeSeries {
    ActivityCount,
    ObjectCount,
    UniqueObjectCount,
    ObjectAttributeOperator,
    EventAttributeOperator
}


pub struct TimeSeriesConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<TimeSeries, Option<Value>>
}

