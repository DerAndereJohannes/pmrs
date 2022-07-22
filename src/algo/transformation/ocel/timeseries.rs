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


pub fn equal_time_binning(log: &Ocel) -> Vec<i64> {
    let bin_series: Vec<i64> = log.events.iter().map(|(_, values)| values.timestamp.timestamp_millis()).collect();
    let time_width: i64 = (bin_series.last().expect("") - bin_series.first().expect("")) / bin_series.len() as i64;

    time_bins(bin_series, time_width)
}

pub fn equal_timediff_binning(log: &Ocel, timediff: i64) -> Vec<i64> {
    let bin_series: Vec<i64> = log.events.iter().map(|(_, values)| values.timestamp.timestamp_millis()).collect();
    
    time_bins(bin_series, timediff)
}


pub fn time_bins(series: Vec<i64>, timediff: i64) -> Vec<i64> {
    let first_time: i64 = series.first().expect("").to_owned();
    series.iter().map(|millis| (millis - first_time) / timediff).collect()
}

pub fn remove_bin_gaps(bin_list: Vec<i64>) -> Vec<i64> {
    let mut curr_num: i64 = 0;
    let mut prev_bin = bin_list.first().unwrap().to_owned();
    bin_list.iter().map(|bin_id| { if bin_id != &prev_bin {curr_num += 1}; prev_bin = *bin_id; curr_num}).collect()
}
