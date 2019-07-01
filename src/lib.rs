// Copyright © 2019 Jubal Gonzalez-Santos
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(2.0), mean(&[1.0, 2.0, 3.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
    let count = nums.len() as f64;
    let sum = nums.iter().sum::<f64>() as f64;

    if nums.is_empty() {
        Some(0.0)
    } else {
        Some(sum / count)
    }
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(1.5), stddev(&[1.0, -2.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    match (mean(nums), nums.len()) {
        (Some(nums_mean), count) if count > 0 => {
            let variance = nums
                .iter()
                .map(|value| {
                    let v = *value as f64;
                    let difference = nums_mean - v;
                    difference.powf(2.0)
                })
                .sum::<f64>()
                / count as f64;
            Some(variance.sqrt())
        }
        _ => None,
    }
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.5), median(&[0.0, 0.5, -1.0, 1.0, 2.0]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let s = nums.len();
    if !nums.is_empty() {
        Some(nums[(s - 1) / 2])
    } else {
        None
    }
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(8.0), l2(&[4.0, 4.0, 4.0, 4.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    let mut nums = nums.to_owned();

    if nums.is_empty() {
        Some(0.0)
    } else {
        let mut sum: f64 = 0.0;
        for numbers in &mut nums {
            sum += (*numbers).powf(2.0);
        }
        Some(sum.sqrt())
    }
}
