use std::collections::HashMap;

use ahash::AHashSet;
use nohash_hasher::IntMap;
use serde_json::Value;
use strum::{EnumString, IntoStaticStr, Display};
use num_traits::Num;
use num_traits::{FromPrimitive, ToPrimitive};


use crate::objects::ocel::{Ocel, OcelEvent};


pub enum BinMethod {
    EqualTime,
    EqualTimeDiff
}

impl BinMethod {
    pub fn execute(&self, log: &Ocel, timediff: Option<i64>) -> Vec<i64> {
        match self {
            BinMethod::EqualTime => {
                return auto_timediff_binning(log);
            },
            BinMethod::EqualTimeDiff => {
                if let Some(td) = timediff {
                    return manual_timediff_binning(log, td);
                }
                return vec![];
            }
        }
    }
}

#[derive(EnumString, IntoStaticStr, Display, Hash, Eq, PartialEq)]
pub enum TimeSeries {
    ActivityCount,
    ObjectCount,
    UniqueObjectCount,
    ObjectAttributeOperator,
    EventAttributeOperator
}

impl TimeSeries {
    pub fn execute<T>(&self, _log: &Ocel, series: Vec<&OcelEvent>) -> Option<T> 
        where T: Num + ToPrimitive + PartialOrd + Clone + FromPrimitive {
        match self {
            TimeSeries::ActivityCount => {
                T::from_usize(series.len())
            },
            TimeSeries::ObjectCount => {
                T::from_usize(series.iter().map(|ev| ev.omap.len()).sum())
            },
            TimeSeries::UniqueObjectCount => {
                let mut obj_set = AHashSet::<usize>::new();
                series.iter().for_each(|ev| obj_set.extend(&ev.omap));
                T::from_usize(obj_set.len())
            },
            _ => {
                T::from_f64(1.0)
            }
        }
    }
}


pub struct TimeSeriesConfig<'a> {
    pub ocel: &'a Ocel,
    pub params: &'a HashMap<TimeSeries, Option<Value>>
}


pub fn auto_timediff_binning(log: &Ocel) -> Vec<i64> {
    let bin_series: Vec<i64> = log.events.iter().map(|(_, values)| values.timestamp.timestamp_millis()).collect();
    let time_width: i64 = (bin_series.last().expect("") - bin_series.first().expect("")) / bin_series.len() as i64;

    time_bins(bin_series, time_width)
}

pub fn manual_timediff_binning(log: &Ocel, timediff: i64) -> Vec<i64> {
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


pub fn generate_time_series<T>(log: &Ocel, binning: Vec<i64>, ts_type: TimeSeries) -> Vec<f64>
        where T: Num + ToPrimitive + PartialOrd + Clone + FromPrimitive {
        let binned_series: Vec<i64> = binning;
        
        if binned_series.len() == 0 {
            return vec![];
        }

        let mut curr_id: &i64 = binned_series.first().unwrap();
        let mut final_series: Vec<f64> = vec![];
        let mut head: i64 = 0;
        let mut tail: i64 = 0;

        for bin in binned_series.iter() {
            if bin != curr_id {
                match execute_time_series(&log, head, tail, &ts_type) {
                    Some(res) => final_series.push(res),
                    None => final_series.push(0.0)
                }
                curr_id = bin;
                head = tail;
            }
            tail += 1;
        }

        // insert the remining elements in the series
        match execute_time_series(&log, head, tail, &ts_type) {
            Some(res) => final_series.push(res),
            None => final_series.push(0.0)
        }

        
        final_series
}

fn execute_time_series<'a, T>(log: &'a Ocel, head: i64, tail: i64, ts_type: &TimeSeries) -> Option<T>
        where T: Num + ToPrimitive + PartialOrd + Clone + FromPrimitive {
            let series = get_series(&log.events, head, tail - 1);
            ts_type.execute(log, series)
}


fn get_series<'a>(events: &'a IntMap<usize, OcelEvent>, head: i64, tail: i64) -> Vec<&'a OcelEvent> {
    (head..=tail).into_iter().map(|x| events.get(&(x as usize)).unwrap()).collect()
}


pub fn series_differences_absolute<T>(series: Vec<T>) -> Vec<T>
        where T: Num + ToPrimitive + PartialOrd + Clone + Copy + FromPrimitive {
            let mut diff_series = vec![T::from_f64(0.0).unwrap(); series.len() - 1];
            for (i, item) in diff_series.iter_mut().enumerate() {
                *item = series[i+1] - series[i];
            }
            diff_series
}

pub fn series_differences_relative<T>(series: Vec<T>) -> Vec<f64>
        where T: Num + ToPrimitive + PartialOrd + Clone + Copy + FromPrimitive {
            let mut diff_series: Vec<f64> = vec![0.0; series.len() - 1];
            for (i, item) in diff_series.iter_mut().enumerate() {
                *item = T::to_f64(&(series[i+1] - series[i])).unwrap() / T::to_f64(&series[i]).unwrap();
            }
            diff_series
}



#[cfg(test)]
mod tests {
    use crate::objects::ocel::importer::import_ocel;

    use super::*;

    fn get_test_data() -> Ocel {
        import_ocel("logs/ocel-transformation-test.jsonocel").expect("did you mess with ocel-transformation-test.jsonocel?")
    }

    #[test]
    fn test_generate_time_series() {
        let log = get_test_data();
        let binning = auto_timediff_binning(&log);
        
        // return empty if binning is empty
        assert_eq!(Vec::<f64>::new(), generate_time_series::<f64>(&log, vec![], TimeSeries::ActivityCount));
        // each event is separate, each time bin has 1 event
        assert_eq!(vec![1.0, 1.0, 1.0], generate_time_series::<f64>(&log, binning.clone(), TimeSeries::ActivityCount));
        // each event is separate, each time bin has 1 omap with 2 objects each
        assert_eq!(vec![2.0, 2.0, 2.0], generate_time_series::<f64>(&log, binning, TimeSeries::ObjectCount));
        // time binning is 2 hours, 2 events in first with 3 uniques and 1 event in second with 2
        // uniques
        assert_eq!(vec![3.0, 2.0], generate_time_series::<f64>(&log, manual_timediff_binning(&log, 7200000), TimeSeries::UniqueObjectCount));

    }

    #[test]
    fn test_auto_timediff_binning() {
        let log = get_test_data();
        assert_eq!(vec![0, 1, 3], auto_timediff_binning(&log)); // 2 hours divided into 3
    }

    #[test]
    fn test_manual_timediff_binning() {
        let log = get_test_data();

        assert_eq!(vec![0, 1, 2], manual_timediff_binning(&log, 3600000)); // 1 hour
        assert_eq!(vec![0, 2, 4], manual_timediff_binning(&log, 1800000)); // 0.5 hour
    }

    #[test]
    fn test_get_series() {
        let log = get_test_data();
        let all_events: Vec<&OcelEvent> = log.events.values().collect();
        let first_event: Vec<&OcelEvent> = vec![&log.events[&0]];
        let final_two_events: Vec<&OcelEvent> = vec![&log.events[&1], &log.events[&2]];

        assert_eq!(all_events, get_series(&log.events, 0, 2));
        assert_eq!(first_event, get_series(&log.events, 0, 0));
        assert_eq!(final_two_events, get_series(&log.events, 1, 2));

    }
    
    #[test]
    fn test_timebins() {
        let millis: Vec<i64> = vec![0, 1000, 2000, 3000, 10000];
        assert_eq!(vec![0, 2, 4, 6, 20], time_bins(millis.clone(), 500));
        assert_eq!(vec![0, 0, 1, 1, 5], time_bins(millis.clone(), 2000));
        assert_eq!(vec![0, 0, 0, 0, 1], time_bins(millis, 10000));
    }

    #[test]
    fn test_remove_bin_gaps() {
        let millis: Vec<i64> = vec![0, 1000, 2000, 3000, 10000];
        assert_eq!(vec![0, 1, 2, 3, 4], remove_bin_gaps(time_bins(millis.clone(), 500)));
        assert_eq!(vec![0, 0, 1, 1, 2], remove_bin_gaps(time_bins(millis, 2000)));
    }

    #[test]
    fn test_series_differences_absolute_int() {
        let testi64: Vec<i64> = vec![1,2,3];
        assert_eq!(vec![1,1], series_differences_absolute(testi64));
    }

    #[test]
    fn test_series_differences_absolute_float() {
        let testf64: Vec<f64> = vec![1.5,2.3,3.7];
        let result: Vec<f64> = series_differences_absolute(testf64).iter().map(|x| (x * 100.0).round() / 100.0).collect();
        assert_eq!(vec![0.8,1.4], result);
    }

    #[test]
    fn test_series_differences_relative_int() {
        let testi64: Vec<i64> = vec![1,2,3];
        let result: Vec<f64> = series_differences_relative(testi64).iter().map(|x| (x * 100.0).round() / 100.0).collect();
        assert_eq!(vec![1.0,0.5], result);
    }

    #[test]
    fn test_series_differences_relative_float() {
        let testf64: Vec<f64> = vec![1.5,2.3,3.7];
        let result: Vec<f64> = series_differences_relative(testf64).iter().map(|x| (x * 100.0).round() / 100.0).collect();
        assert_eq!(vec![0.53,0.61], result);
    }



    #[test]
    fn test_time_series_activity_count() {
        let log = get_test_data();
        assert_eq!(3, TimeSeries::ActivityCount.execute::<i64>(&log, log.events.values().into_iter().collect()).unwrap());
    }

    #[test]
    fn test_time_series_object_count() {
        let log = get_test_data();
        assert_eq!(6, TimeSeries::ObjectCount.execute::<i64>(&log, log.events.values().into_iter().collect()).unwrap());
    }

    #[test]
    fn test_time_series_unique_object_count() {
        let log = get_test_data();
        assert_eq!(4, TimeSeries::UniqueObjectCount.execute::<i64>(&log, log.events.values().into_iter().collect()).unwrap());
    }


}
