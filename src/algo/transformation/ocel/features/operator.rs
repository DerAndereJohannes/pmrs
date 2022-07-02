use std::str::FromStr;
use std::iter::Iterator;
use num_traits::Num;
use num_traits::{FromPrimitive, ToPrimitive};
use stats;

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


impl FromStr for Operator {
    type Err = ();

    fn from_str(feature: &str) -> Result<Operator, Self::Err> {
        match feature {
            "Mean" => Ok(Operator::Mean),
            "Median" => Ok(Operator::Median),
            "Mode" => Ok(Operator::Mode),
            "StdDev" => Ok(Operator::StdDev),
            "Variance" => Ok(Operator::Variance),
            "Min" => Ok(Operator::Min),
            "Max" => Ok(Operator::Max),
            "Count" => Ok(Operator::Count),
            _ => Err(())
        }
    }
}

fn operator_mean<I, T>(input: I) -> T 
where I: Iterator, T: Num + FromPrimitive, <I as Iterator>::Item: ToPrimitive {
    T::from_f64(stats::mean(input)).unwrap()
}


fn operator_median<I, T>(input: I) -> T 
where T: Num + FromPrimitive, <I as Iterator>::Item: ToPrimitive + PartialOrd, I: Iterator {
    T::from_f64(stats::median(input).unwrap()).unwrap()
}
