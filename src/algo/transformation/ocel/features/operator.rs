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


#[cfg(test)]
mod tests {
    use super::*;
    const ERROR: f64 = 0.00001;

    fn get_int_ord_vec() -> Vec<i32>{
        vec![1,3,5,7,9]
    }

    fn get_int_parord_vec() -> Vec<i32>{
        vec![1,3,5,5,9]
    }

    fn get_float_ord_vec() -> Vec<f64> {
        vec![1.5, 2.4, 3.3, 4.2, 5.1]
    }

    fn get_float_parord_vec() -> Vec<f64> {
        vec![1.5, 2.4, 3.3, 3.3, 5.1]
    }


    #[test]
    fn test_mean_operator() {
        let op = Operator::Mean;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();
        
        assert!((op.execute(iov.iter().map(|i| *i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert!((op.execute(ipv.iter().map(|i| *i as f64)).unwrap().to_owned() - 4.6).abs() < ERROR);
        assert!((op.execute(fov.iter().map(|i| *i)).unwrap().to_owned() - 3.3).abs() < ERROR);
        assert!((op.execute(fpv.iter().map(|i| *i)).unwrap().to_owned() - 3.12).abs() < ERROR);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();
        
        assert!((op.execute(riov.iter().map(|i| **i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert!((op.execute(ripv.iter().map(|i| **i as f64)).unwrap().to_owned() - 4.6).abs() < ERROR);
        assert!((op.execute(rfov.iter().map(|i| **i)).unwrap().to_owned() - 3.3).abs() < ERROR);
        assert!((op.execute(rfpv.iter().map(|i| **i)).unwrap().to_owned() - 3.12).abs() < ERROR);
    }

    #[test]
    fn test_median_operator() {
        let op = Operator::Median;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();
        
        assert!((op.execute(iov.iter().map(|i| *i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert!((op.execute(ipv.iter().map(|i| *i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert!((op.execute(fov.iter().map(|i| *i)).unwrap().to_owned() - 3.3).abs() < ERROR);
        assert!((op.execute(fpv.iter().map(|i| *i)).unwrap().to_owned() - 3.3).abs() < ERROR);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();
        
        assert!((op.execute(riov.iter().map(|i| **i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert!((op.execute(ripv.iter().map(|i| **i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert!((op.execute(rfov.iter().map(|i| **i)).unwrap().to_owned() - 3.3).abs() < ERROR);
        assert!((op.execute(rfpv.iter().map(|i| **i)).unwrap().to_owned() - 3.3).abs() < ERROR);
    }

    #[test]
    fn test_mode_operator() {
        let op = Operator::Mode;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();
        
        assert_eq!(op.execute(iov.iter().map(|i| *i as f64)), None);
        assert!((op.execute(ipv.iter().map(|i| *i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert_eq!(op.execute(fov.iter().map(|i| *i as f64)), None);
        assert!((op.execute(fpv.iter().map(|i| *i)).unwrap().to_owned() - 3.3).abs() < ERROR);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();
        
        assert_eq!(op.execute(riov.iter().map(|i| **i as f64)), None);
        assert!((op.execute(ripv.iter().map(|i| **i as f64)).unwrap().to_owned() - 5.0).abs() < ERROR);
        assert_eq!(op.execute(rfov.iter().map(|i| **i as f64)), None);
        assert!((op.execute(rfpv.iter().map(|i| **i)).unwrap().to_owned() - 3.3).abs() < ERROR);
    }

    #[test]
    fn test_stddev_operator() {
        let op = Operator::StdDev;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();

        assert!((op.execute(iov.iter().map(|i| *i as f64)).unwrap().to_owned() - 2.82842712).abs() < ERROR);
        assert!((op.execute(ipv.iter().map(|i| *i as f64)).unwrap().to_owned() - 2.65329983).abs() < ERROR);
        assert!((op.execute(fov.iter().map(|i| *i)).unwrap().to_owned() - 1.2727922061).abs() < ERROR);
        assert!((op.execute(fpv.iter().map(|i| *i)).unwrap().to_owned() - 1.1939849245).abs() < ERROR);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();
        
        assert!((op.execute(riov.iter().map(|i| **i as f64)).unwrap().to_owned() - 2.82842712).abs() < ERROR);
        assert!((op.execute(ripv.iter().map(|i| **i as f64)).unwrap().to_owned() - 2.65329983).abs() < ERROR);
        assert!((op.execute(rfov.iter().map(|i| **i)).unwrap().to_owned() - 1.2727922061).abs() < ERROR);
        assert!((op.execute(rfpv.iter().map(|i| **i)).unwrap().to_owned() - 1.1939849245).abs() < ERROR);
    }

    #[test]
    fn test_variance_operator() {
        let op = Operator::Variance;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();

        assert!((op.execute(iov.iter().map(|i| *i as f64)).unwrap().to_owned() - 8.0).abs() < ERROR);
        assert!((op.execute(ipv.iter().map(|i| *i as f64)).unwrap().to_owned() - 7.04).abs() < ERROR);
        assert!((op.execute(fov.iter().map(|i| *i)).unwrap().to_owned() - 1.62).abs() < ERROR);
        assert!((op.execute(fpv.iter().map(|i| *i)).unwrap().to_owned() - 1.4256).abs() < ERROR);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();
        
        assert!((op.execute(riov.iter().map(|i| **i as f64)).unwrap().to_owned() - 8.0).abs() < ERROR);
        assert!((op.execute(ripv.iter().map(|i| **i as f64)).unwrap().to_owned() - 7.04).abs() < ERROR);
        assert!((op.execute(rfov.iter().map(|i| **i)).unwrap().to_owned() - 1.62).abs() < ERROR);
        assert!((op.execute(rfpv.iter().map(|i| **i)).unwrap().to_owned() - 1.4256).abs() < ERROR);
    }

    #[test]
    fn test_min_operator() {
        let op = Operator::Min;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();
        
        assert_eq!(op.execute(iov.iter().map(|i| *i as f64)).unwrap() as i32, 1);
        assert_eq!(op.execute(ipv.iter().map(|i| *i as f64)).unwrap() as i32, 1);
        assert_eq!(op.execute(fov.iter().map(|i| *i as f64)).unwrap(), 1.5);
        assert_eq!(op.execute(fpv.iter().map(|i| *i as f64)).unwrap(), 1.5);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();

        assert_eq!(op.execute(riov.iter().map(|i| **i as f64)).unwrap() as i32, 1);
        assert_eq!(op.execute(ripv.iter().map(|i| **i as f64)).unwrap() as i32, 1);
        assert_eq!(op.execute(rfov.iter().map(|i| **i as f64)).unwrap(), 1.5);
        assert_eq!(op.execute(rfpv.iter().map(|i| **i as f64)).unwrap(), 1.5);
    }

    #[test]
    fn test_max_operator() {
        let op = Operator::Max;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();
        
        assert_eq!(op.execute(iov.iter().map(|i| *i as f64)).unwrap() as i32, 9);
        assert_eq!(op.execute(ipv.iter().map(|i| *i as f64)).unwrap() as i32, 9);
        assert_eq!(op.execute(fov.iter().map(|i| *i as f64)).unwrap(), 5.1);
        assert_eq!(op.execute(fpv.iter().map(|i| *i as f64)).unwrap(), 5.1);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();

        assert_eq!(op.execute(riov.iter().map(|i| **i as f64)).unwrap() as i32, 9);
        assert_eq!(op.execute(ripv.iter().map(|i| **i as f64)).unwrap() as i32, 9);
        assert_eq!(op.execute(rfov.iter().map(|i| **i as f64)).unwrap(), 5.1);
        assert_eq!(op.execute(rfpv.iter().map(|i| **i as f64)).unwrap(), 5.1);
    }

    #[test]
    fn test_count_operator() {
        let op = Operator::Count;

        let iov = get_int_ord_vec();
        let ipv = get_int_parord_vec();
        let fov = get_float_ord_vec();
        let fpv = get_float_parord_vec();
        
        assert_eq!(op.execute(iov.iter().map(|i| *i as usize)).unwrap(), 5);
        assert_eq!(op.execute(ipv.iter().map(|i| *i as usize)).unwrap(), 5);
        assert_eq!(op.execute(fov.iter().map(|i| *i as usize)).unwrap(), 5);
        assert_eq!(op.execute(fpv.iter().map(|i| *i as usize)).unwrap(), 5);

        // mess with ordering
        let riov: Vec<&i32> = iov.iter().rev().collect();
        let ripv: Vec<&i32> = ipv.iter().rev().collect();
        let rfov: Vec<&f64> = fov.iter().rev().collect();
        let rfpv: Vec<&f64> = fpv.iter().rev().collect();

        assert_eq!(op.execute(riov.iter().map(|i| **i as usize)).unwrap(), 5);
        assert_eq!(op.execute(ripv.iter().map(|i| **i as usize)).unwrap(), 5);
        assert_eq!(op.execute(rfov.iter().map(|i| **i as usize)).unwrap(), 5);
        assert_eq!(op.execute(rfpv.iter().map(|i| **i as usize)).unwrap(), 5);
    }
}
