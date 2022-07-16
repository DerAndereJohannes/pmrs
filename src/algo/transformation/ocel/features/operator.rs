use std::iter::Iterator;
use itertools::Itertools;
use num_traits::Num;
use num_traits::{FromPrimitive, ToPrimitive};
use stats;
use strum::EnumString;

#[derive(Debug, EnumString)]
pub enum Operator {
    Mean,
    Median,
    Mode,
    StdDev,
    Variance,
    Min,
    Max,
    Count
}


impl Operator {
    pub fn execute<T, I>(&self, input_iter: I) -> Option<T> 
        where I: Iterator<Item = T>, T: Num + ToPrimitive + PartialOrd + Clone + FromPrimitive {
        
        match self {
            Operator::Mean => {
                T::from_f64(stats::mean(input_iter))
            }, 
            Operator::Median => {
                let ordered_iterator = input_iter.sorted_by(|a,b| a.partial_cmp(b).expect("A NaN value got into the iterator!"));
                T::from_f64(stats::median(ordered_iterator).unwrap())
            },
            // any order
            Operator::Mode => {
                stats::mode(input_iter)
                // match stats::mode(input_iter) {
                //     Some(it_mode) => {
                //         return it_mode;
                //     },
                //     None => {return T::zero();}
                // }
                
            },
            Operator::StdDev => {
                T::from_f64(stats::stddev(input_iter))
            },
            Operator::Variance => {
                T::from_f64(stats::variance(input_iter))
            },
            Operator::Min => {
                input_iter.min_by(|a, b| a.partial_cmp(b).expect("A NaN value got into the iterator!"))
            },
            Operator::Max => {
                input_iter.max_by(|a, b| a.partial_cmp(b).expect("A NaN value got into the iterator!"))
            },
            Operator::Count => {
                T::from_usize(input_iter.count())
            }
        }
    }
    
}

