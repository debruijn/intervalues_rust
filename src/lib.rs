//! # Intervalues
//!
//! `intervalues` brings functionality to combine valued intervals together in an efficient manner.


use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

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
/// Note: currently only integers supported as interval bounds - floats will follow.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues;
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[isize; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let out: HashMap<(isize, isize), isize> = intervalues::combine_intervals_values(input);
///
/// // 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
/// assert_eq!(out[&(0, 1)], 1);
/// assert_eq!(out[&(1, 2)], 3);
/// assert_eq!(out[&(2, 3)], 2);
/// ```
pub fn combine_intervals_values(raw_ivs: Vec<[isize; 3]>) -> HashMap<(isize, isize), isize> {
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
/// Note: currently only integers supported as interval bounds - floats will follow.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues;
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[isize; 2]> = vec!([0, 2], [1, 3]);
/// let out: HashMap<(isize, isize), isize> = intervalues::combine_intervals(input);
///
/// // 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
/// assert_eq!(out[&(0, 1)], 1);
/// assert_eq!(out[&(1, 2)], 2);
/// assert_eq!(out[&(2, 3)], 1);
/// ```
pub fn combine_intervals(raw_ivs: Vec<[isize; 2]>) -> HashMap<(isize, isize), isize> {
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


fn intervals_to_points_flt(input: Vec<[f64; 2]>) -> Vec<(Decimal, isize)> {
    let mut out: DefaultHashMap<Decimal, isize> = DefaultHashMap::new();
    for entry in input.iter() {
        let mult = if entry[0] > entry[1] { -1 } else { 1 };
        out[Decimal::from_f64_retain(entry[0]).unwrap()] += mult;
        out[Decimal::from_f64_retain(entry[1]).unwrap()] -= mult;
    }
    let mut out: Vec<(Decimal, isize)> = out
        .iter()
        .filter(|x| *x.1 != 0)
        .map(|x| (*x.0, *x.1))
        .collect();
    out.sort();
    out
}

/// Combine intervals with counts to an efficient and reduced collection.
/// Note: currently only integers supported as interval bounds - floats will follow.
///
/// # Examples
///
/// ```
/// use intervalues;
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[f64; 2]> = vec!([0.0, 2.0], [1.0, 3.0]);
/// let out: Vec<(f64, f64, isize)> = intervalues::combine_intervals_flt(input);
///
/// // 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
/// assert_eq!(out[0], (0.0, 1.0, 1));
/// assert_eq!(out.iter().map(|x| x.2).collect(), (1, 2, 1));
/// ```
pub fn combine_intervals_flt(raw_ivs: Vec<[f64; 2]>) -> Vec<(f64, f64, isize)> {
    // Convert input intervals to point counts
    let endpoints = intervals_to_points_flt(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = 0;
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    // let mut out = HashMap::new();
    let mut out = Vec::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 != 0 {
            out.push((lb.0.to_f64().unwrap(), ub.0.to_f64().unwrap(), lb.1));
        }
    }
    out
}


fn intervals_values_to_points_flt(input: Vec<[f64; 3]>) -> Vec<(Decimal, f64)> {
    let mut out: DefaultHashMap<Decimal, f64> = DefaultHashMap::new();
    for entry in input.iter() {
        let mult = if entry[0] > entry[1] { -1.0 } else { 1.0 };
        out[Decimal::from_f64_retain(entry[0]).unwrap()] += mult * entry[2];
        out[Decimal::from_f64_retain(entry[1]).unwrap()] -= mult * entry[2];
    }
    let mut out: Vec<(Decimal, f64)> = out
        .iter()
        .filter(|x| *x.1 != 0.0)
        .map(|x| (*x.0, *x.1))
        .collect();
    out.sort_by_key(|x| x.0);
    out
}

pub fn combine_intervals_flt_values(raw_ivs: Vec<[f64; 3]>) -> Vec<(f64, f64, f64)> {
    // Convert input intervals to point counts
    let endpoints = intervals_values_to_points_flt(raw_ivs);

    // Convert point counts to cumulative point counts
    let mut curr_val = 0.0;
    let mut new_map = Vec::new();
    for pt in endpoints {
        curr_val += pt.1;
        new_map.push((pt.0, curr_val))
    }

    // Convert cumulative point counts to intervals
    // let mut out = HashMap::new();
    let mut out = Vec::new();
    for (lb, ub) in new_map.iter().tuple_windows() {
        if lb.1 != 0.0 {
            out.push((lb.0.to_f64().unwrap(), ub.0.to_f64().unwrap(), lb.1));
        }
    }
    out
}



#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn different_counts() {
    //     let input: Vec<[isize; 3]> = vec!([0, 2, 1], [1, 3, 2]);
    //     let out: HashMap<(isize, isize), isize> = combine_intervals_values(input);
    //
    //     // {(0, 1): 1, (2, 3): 2, (1, 2): 3}
    //     assert_eq!(out[&(0, 1)], 1);
    //     assert_eq!(out[&(1, 2)], 3);
    //     assert_eq!(out[&(2, 3)], 2);
    // }

}
