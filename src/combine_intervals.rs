use crate::base_interval::BaseInterval;
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use number_general::{Float, Number};
use rust_decimal::Decimal;
use safecast::CastFrom;
use std::collections::HashMap;
use crate::IntervalCollection;

fn intervals_values_to_points(input: Vec<[isize; 3]>) -> Vec<(isize, isize)> {
    let mut out: DefaultHashMap<isize, isize> = DefaultHashMap::new();
    for entry in input.iter() {
        let mult = if entry[0] > entry[1] { -1 } else { 1 };
        out[entry[0]] += mult * entry[2];
        out[entry[1]] -= mult * entry[2];
    }
    let mut out: Vec<(isize, isize)> = out
        .iter()
        .filter(|x| *x.1 != 0)
        .map(|x| (*x.0, *x.1))
        .collect();
    out.sort();
    out
}

fn intervals_to_points(input: Vec<[isize; 2]>) -> Vec<(isize, isize)> {
    let mut out: DefaultHashMap<isize, isize> = DefaultHashMap::new();
    for entry in input.iter() {
        let mult = if entry[0] > entry[1] { -1 } else { 1 };
        out[entry[0]] += mult;
        out[entry[1]] -= mult;
    }
    let mut out: Vec<(isize, isize)> = out
        .iter()
        .filter(|x| *x.1 != 0)
        .map(|x| (*x.0, *x.1))
        .collect();
    out.sort();
    out
}

/// Combine intervals with values to an efficient and reduced collection.
/// This is the isize implementation for valued intervals.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues;
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[isize; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let out: HashMap<(isize, isize), isize> = intervalues::combine_intervals_isize(input);
///
/// // 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
/// assert_eq!(out[&(0, 1)], 1);
/// assert_eq!(out[&(1, 2)], 3);
/// assert_eq!(out[&(2, 3)], 2);
/// ```
pub fn combine_intervals_isize(raw_ivs: Vec<[isize; 3]>) -> HashMap<(isize, isize), isize> {
    // Convert input intervals to point counts
    let endpoints = intervals_values_to_points(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = 0;
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    let mut out = HashMap::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 != 0 {
            out.insert((lb.0, ub.0), lb.1);
        }
    }
    out
}

/// Combine intervals with counts to an efficient and reduced collection.
/// This is the isize implementation for unvalued intervals.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues;
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[isize; 2]> = vec!([0, 2], [1, 3]);
/// let out: HashMap<(isize, isize), isize> = intervalues::combine_intervals_isize_no_val(input);
///
/// // 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
/// assert_eq!(out[&(0, 1)], 1);
/// assert_eq!(out[&(1, 2)], 2);
/// assert_eq!(out[&(2, 3)], 1);
/// ```
pub fn combine_intervals_isize_no_val(raw_ivs: Vec<[isize; 2]>) -> HashMap<(isize, isize), isize> {
    // Convert input intervals to point counts
    let endpoints = intervals_to_points(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = 0;
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    let mut out = HashMap::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 != 0 {
            out.insert((lb.0, ub.0), lb.1);
        }
    }
    out
}

fn base_intervals_to_points(input: Vec<BaseInterval>) -> Vec<(Number, Number)> {
    let mut out: DefaultHashMap<Number, Number> = DefaultHashMap::new();
    for entry in input.iter() {
        out[entry.get_lb()] += entry.get_value();
        out[entry.get_ub()] -= entry.get_value();
    }
    let mut out: Vec<(Number, Number)> = out
        .iter()
        .filter(|x| *x.1 != Number::from(0.0))
        .map(|x| (*x.0, *x.1))
        .collect();
    out.sort_by_key(|x| Decimal::from_f64_retain(f64::cast_from(Float::cast_from(x.0))));
    out
}

/// Combine intervals with values to an efficient and reduced collection.
/// This is the BaseInterval implementation for valued intervals - which is the main implementation.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use number_general::Number;
/// use intervalues;
/// use intervalues::{BaseInterval, IntervalCollection};
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[i64; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let input = input.iter()
///     .map(|x| BaseInterval::new(Number::from(x[0]), Number::from(x[1]), Number::from(x[2])))
///     .collect();
/// let out: IntervalCollection = intervalues::combine_intervals(input);
///
/// // 'out' = {(0, 1, 1), (2, 3, 2), (1, 2, 3)}
/// assert_eq!(out.to_vec_as_counter()[0], BaseInterval::default());
/// assert_eq!(out.to_vec_owned()[1], BaseInterval::new(Number::from(1), Number::from(2), Number::from(3)));
/// ```
pub fn combine_intervals(raw_ivs: Vec<BaseInterval>) -> IntervalCollection {
    let endpoints: Vec<(Number, Number)> = base_intervals_to_points(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = Number::from(0);
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    let mut out = Vec::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 != Number::from(0.0) {
            out.push(BaseInterval::new(lb.0, ub.0, lb.1));
        }
    }
    IntervalCollection::from_vec(out)
}
