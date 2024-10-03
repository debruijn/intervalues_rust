//! # Intervalues
//!
//! `intervalues` brings functionality to combine valued intervals together in an efficient manner.



use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::collections::HashMap;

fn intervals_to_points(input: Vec<[isize; 3]>) -> Vec<(isize, isize)> {
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
/// let input: Vec<[isize; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let out: HashMap<(isize, isize), isize> = intervalues::combine_intervals(input);
///
/// // 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
/// assert_eq!(out[&(0, 1)], 1);
/// assert_eq!(out[&(1, 2)], 3);
/// assert_eq!(out[&(2, 3)], 2);
/// ```
pub fn combine_intervals(raw_ivs: Vec<[isize; 3]>) -> HashMap<(isize, isize), isize> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let input: Vec<[isize; 3]> = vec!([0, 2, 1], [1, 3, 2]);
        let out: HashMap<(isize, isize), isize> = combine_intervals(input);

        // {(0, 1): 1, (2, 3): 2, (1, 2): 3}
        assert_eq!(out[&(0, 1)], 1);
        assert_eq!(out[&(1, 2)], 3);
        assert_eq!(out[&(2, 3)], 2);
    }

}
