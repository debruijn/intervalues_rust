use crate::interval::Interval;
use crate::{BaseInterval, IntervalCollection};
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use num_traits::{Num, ToPrimitive};
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{AddAssign, SubAssign};

fn intervals_to_points<T, U>(input: Vec<Interval<T, U>>) -> Vec<(T, U)>
where
    T: Num + PartialOrd + Clone + Eq + Hash + Copy + Display + Debug,
    U: Num + PartialOrd + Default + AddAssign + SubAssign + Clone + Copy + Display + Debug,
{
    let mut out: DefaultHashMap<T, U> = DefaultHashMap::new();
    for entry in input.iter() {
        let this = entry.to_tuple();
        out[this.0] += this.2;
        out[this.1] -= this.2;
    }
    let mut out: Vec<(T, U)> = out
        .iter()
        .filter(|x| *x.1 != U::zero())
        .map(|x| (x.0.to_owned(), x.1.to_owned()))
        .collect();
    out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    out
}

/// Combine intervals with values to an efficient and reduced collection, taking overlaps and
/// duplicates into account.
/// Returns an IntervalCollection struct which can be converted further.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues::{Interval, IntervalCollection, combine_intervals};
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[i64; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let input = input.iter()
///     .map(|x| Interval::new(x[0], x[1], x[2]))
///     .collect();
/// let out: IntervalCollection<i64,i64> = combine_intervals(input);
///
/// // 'out' = {(0, 1, 1), (2, 3, 2), (1, 2, 3)}
/// assert_eq!(out.to_vec_as_counter()[0], Interval::default());
/// assert_eq!(out.to_vec_owned()[1], Interval::new(1, 2, 3));
/// ```
pub fn combine_intervals<T, U>(raw_ivs: Vec<Interval<T, U>>) -> IntervalCollection<T, U>
where
    T: Num + PartialOrd + Clone + Hash + Copy + Eq + Display + Debug,
    U: Num
        + PartialOrd
        + Default
        + AddAssign
        + SubAssign
        + Clone
        + Copy
        + ToPrimitive
        + std::iter::Sum
        + Display
        + Debug,
{
    let endpoints: Vec<(T, U)> = intervals_to_points(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = U::zero();
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    let mut out = Vec::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 != U::zero() {
            out.push(Interval::new(lb.0, ub.0, lb.1));
        }
    }
    IntervalCollection::from_vec(out)
}

/// Combine intervals with values to an efficient and reduced collection, taking overlaps and
/// duplicates into account. This version returns the intervals with a positive final value.
/// Returns a Vec of BaseIntervals.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues::{Interval, IntervalCollection, combine_as_set, BaseInterval};
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[i64; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let input = input.iter()
///     .map(|x| Interval::new(x[0], x[1], x[2]))
///     .collect();
/// let out: Vec<BaseInterval<i64>> = combine_as_set(input);
///
/// // 'out' = {(0, 1, 1), (2, 3, 2), (1, 2, 3)}
/// assert_eq!(out[0], BaseInterval::new(0, 3));
/// ```
pub fn combine_as_set<T, U>(raw_ivs: Vec<Interval<T, U>>) -> Vec<BaseInterval<T>>
where
    T: Num + PartialOrd + Clone + Hash + Copy + Eq + Display + Debug,
    U: Num
        + PartialOrd
        + Default
        + AddAssign
        + SubAssign
        + Clone
        + Copy
        + ToPrimitive
        + std::iter::Sum
        + Display
        + Debug,
{
    let endpoints: Vec<(T, U)> = intervals_to_points(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = U::zero();
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    let mut out = Vec::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 > U::zero() {
            match out.last() {
                None => {
                    out.push(BaseInterval::new(lb.0, ub.0));
                }
                Some(x) => {
                    if x.get_ub() == lb.0 {
                        let new_lb = x.get_lb();
                        out.pop();
                        out.push(BaseInterval::new(new_lb, ub.0));
                    } else {
                        out.push(BaseInterval::new(lb.0, ub.0));
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_overlap() {
        let this: Vec<[i64; 3]> = vec![[0, 2, 1], [1, 3, 2]];
        let this = this
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let this = combine_intervals(this);
        let that: Vec<[i64; 3]> = vec![[0, 1, 1], [1, 2, 3], [2, 3, 2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let that = IntervalCollection::from_vec(that);
        assert_eq!(this, that);
    }

    #[test]
    fn test_without_overlap() {
        let this: Vec<[i64; 3]> = vec![[0, 1, 1], [2, 3, 2]];
        let this = this
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let this = IntervalCollection::from_vec(this);
        let that: Vec<[i64; 3]> = vec![[0, 1, 1], [2, 3, 2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let that = combine_intervals(that);
        assert_eq!(this, that);
    }

    #[test]
    fn test_created_overlap() {
        let this: Vec<[i64; 3]> = vec![[0, 1, 2], [2, 3, -2]];
        let this = this
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let this = IntervalCollection::from_vec(this);
        let that: Vec<[i64; 3]> = vec![[0, 2, 2], [1, 3, -2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let that = combine_intervals(that);
        assert_eq!(this, that);
    }

    #[test]
    fn test_merge() {
        let this: Vec<[i64; 3]> = vec![[0, 1, 2], [1, 2, 2]];
        let this = this
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let this = combine_intervals(this);
        let that: Vec<[i64; 3]> = vec![[0, 2, 2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let that = IntervalCollection::from_vec(that);
        assert_eq!(this, that);
    }

    #[test]
    fn test_set_with_overlap() {
        let this: Vec<[i64; 3]> = vec![[0, 2, 1], [1, 3, 2]];
        let this = this
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let this = combine_as_set(this);
        let that: Vec<[i64; 2]> = vec![[0, 3]];
        let that: Vec<BaseInterval<i64>> =
            that.iter().map(|x| BaseInterval::new(x[0], x[1])).collect();
        assert_eq!(this, that);
    }

    #[test]
    fn test_set_without_overlap() {
        let this: Vec<[i64; 2]> = vec![[0, 1], [2, 3]];
        let this: Vec<BaseInterval<i64>> = this
            .iter()
            .map(|x| BaseInterval::new(x[0], x[1]))
            .collect();
        let that: Vec<[i64; 3]> = vec![[0, 1, 1], [2, 3, 2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let that = combine_as_set(that);
        assert_eq!(this, that);
    }

    #[test]
    fn test_as_set_both_impls() {
        let that: Vec<[i64; 3]> = vec![[0, 2, 1], [1, 3, 2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let this = combine_as_set(that);
        let that: Vec<[i64; 3]> = vec![[0, 2, 1], [1, 3, 2]];
        let that = that
            .iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        let that = combine_intervals(that).to_vec_as_set();
        assert_eq!(this, that);
    }
}
