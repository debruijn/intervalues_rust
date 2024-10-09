use crate::{BaseInterval, Interval};
use num_traits::{Num, ToPrimitive};
use safecast::CastInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
/// Result of combine_intervals: a collection of Intervals. This can be converted to a Vec of
/// Intervals, or converted to a Counter (only integer and positive counts instead of values) or
/// Set (any Interval with value >0 is included and if possible combined with neighbouring
/// intervals).
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use intervalues::{Interval, IntervalCollection, combine_intervals, BaseInterval};
///
/// // Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
/// let input: Vec<[i64; 3]> = vec!([0, 2, 1], [1, 3, 2]);
/// let input = input.iter()
///     .map(|x| Interval::new(x[0], x[1], x[2]))
///     .collect();
/// let out: IntervalCollection<i64,i64> = combine_intervals(input);
///
/// assert_eq!(out.to_vec_as_counter()[0], Interval::default());
/// assert_eq!(out.to_vec()[1], Interval::new(1, 2, 3));
/// assert_eq!(out.to_vec_as_set(), vec!(BaseInterval::new(0, 3)))
/// ```
pub struct IntervalCollection<T: Num + PartialOrd + Clone + Display, U: Num + PartialOrd + Display>
{
    intervals: Vec<Interval<T, U>>,
}

impl<T, U> Display for IntervalCollection<T, U>
where
    T: Num + PartialOrd + Clone + Copy + Display,
    U: Num + PartialOrd + Clone + Copy + Display + std::iter::Sum,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T, U> IntervalCollection<T, U>
where
    T: Num + PartialOrd + Clone + Copy + Display,
    U: Num + PartialOrd + Clone + Copy + Display + std::iter::Sum,
{
    // Assumes ICs are always sorted by combine_intervals

    pub fn new() -> Self {
        IntervalCollection {
            intervals: Vec::new(),
        }
    }

    pub fn print(&self) -> String {
        format!(
            "IntervalCollection ({}x between {} and {})",
            self.len(),
            self.get_lb(),
            self.get_ub()
        )
    }

    pub fn get_lb(&self) -> T {
        // Todo: properly deal with empty collection
        self.intervals[0].get_lb()
    }

    pub fn get_ub(&self) -> T {
        // Todo: properly deal with empty collection
        self.intervals.last().unwrap().get_ub()
    }

    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    pub fn get_bounds(&self) -> (T, T) {
        // Todo: properly deal with empty collection
        (
            self.intervals[0].get_lb(),
            self.intervals.last().unwrap().get_ub(),
        )
    }

    pub fn contains_num(&self, num: T) -> bool {
        for interval in self.intervals.iter() {
            if interval.contains(num) {
                return true;
            }
        }
        false
    }

    pub fn get_value(&self, num: T) -> U {
        for interval in self.intervals.iter() {
            if interval.contains(num) {
                return interval.get_value();
            }
        }
        U::zero()
    }

    pub fn contains_interval(&self, interval: Interval<T, U>) -> bool {
        let mut to_check = interval.clone();
        for interval in self.intervals.iter() {
            if interval.superset(to_check) {
                return true;
            } else if &to_check.get_lb() < &interval.get_lb() {
                return false;
            } else if &to_check.get_lb() > &interval.get_ub() {
                continue;
            } else {
                to_check =
                    Interval::new(interval.get_ub(), to_check.get_ub(), to_check.get_value());
            }
        }
        false
    }

    pub fn get_value_of_interval_by_parts( // TODO write test for this and test all cases
        &self,
        interval: Interval<T, U>,
    ) -> IntervalCollection<T, U> {
        let mut values = Vec::new();
        let mut to_check = interval.clone();
        for interval in self.intervals.iter() {
            if interval.superset(to_check) {
                let new = Interval::new(
                    to_check.get_lb(),
                    to_check.get_ub(),
                    interval.get_value() / to_check.get_value(),
                );
                values.push(new);
                return IntervalCollection::from_vec(values);
            } else if &to_check.get_lb() < &interval.get_lb() {
                return IntervalCollection::from_vec(values);
            } else if &to_check.get_lb() > &interval.get_ub() {
                continue;
            } else {
                to_check =
                    Interval::new(interval.get_ub(), to_check.get_ub(), to_check.get_value());
            }
        }
        IntervalCollection::from_vec(values)
    }

    pub fn get_partially_overlaps_interval(&self, other: &Interval<T, U>) -> bool {
        for interval in self.intervals.iter() {
            if interval.overlaps(other) {
                return true;
            }
        }
        false
    }

    pub fn get_partially_overlaps(&self, other: IntervalCollection<T, U>) -> bool {
        for interval in other.intervals.iter() {
            if self.get_partially_overlaps_interval(interval) {
                return true;
            }
        }
        false
    }

    pub fn from_vec(vec: Vec<Interval<T, U>>) -> Self {
        IntervalCollection { intervals: vec }
    }

    pub fn to_vec_owned(self) -> Vec<Interval<T, U>> {
        self.intervals
    }

    pub fn to_vec(&self) -> Vec<Interval<T, U>> {
        self.intervals.clone()
    }

    pub fn to_vec_as_set(&self) -> Vec<BaseInterval<T>> {
        // TODO: create unvalued BI (no U)
        let mut new = Vec::new();
        if self.len() == 0 {
            return new;
        }
        let mut this_interval = self.intervals[0];
        for next_interval in self.intervals[1..].iter() {
            if this_interval.can_join_as_set(next_interval) {
                this_interval = this_interval.join_ign_value(*next_interval);
            } else {
                new.push(this_interval.to_base());
                this_interval = *next_interval;
            }
        }
        new.push(this_interval.to_base());
        new
    }
}

impl<T, U> IntervalCollection<T, U>
where
    T: Num + PartialOrd + Clone + Copy + Display,
    U: Num + PartialOrd + Clone + Copy + std::iter::Sum + From<T> + Display,
{
    pub fn total_value(&self) -> U {
        self.intervals
            .iter()
            .map(|x| <T as CastInto<U>>::cast_into(x.get_width()) * x.get_value())
            .sum()
    }
}

impl<T, U> IntervalCollection<T, U>
where
    T: Num + PartialOrd + Clone + Copy + Display,
    U: Num + PartialOrd + Clone + Copy + ToPrimitive + std::iter::Sum + Display,
{
    pub fn to_vec_as_counter(&self) -> Vec<Interval<T, usize>> {
        let mut new = Vec::new();
        if self.len() == 0 {
            return new;
        }
        let mut this_interval = self.intervals[0].val_to_count();
        for next_interval in self.intervals[1..].iter() {
            let next_count = next_interval.val_to_count();
            if this_interval.can_join(&next_count) {
                this_interval = this_interval.join(next_count);
            } else {
                if this_interval.get_value() >= 1 {
                    new.push(this_interval);
                }
                this_interval = next_count;
            }
        }
        if this_interval.get_value() >= 1 {
            new.push(this_interval);
        }
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combine_intervals;

    fn get_input() -> Vec<Interval<i64, i64>>{
        let input: Vec<[i64; 3]> = vec!([0, 2, 1], [1, 3, 2]);
        let input = input.iter()
            .map(|x| Interval::new(x[0], x[1], x[2]))
            .collect();
        input
    }

    #[test]
    fn test_bounds() {
        let this = IntervalCollection::from_vec(get_input());
        assert_eq!(this.get_lb(), 0);
        assert_eq!(this.get_ub(), 3);
        assert_eq!(this.get_bounds(), (0, 3));
    }

    #[test]
    fn test_contains() {
        let this = combine_intervals::combine_intervals(get_input());
        assert!(this.contains_num(1));
        assert!(this.contains_num(2));
        assert!(this.contains_num(3));
        assert!(!this.contains_num(4));
        assert!(!this.contains_num(-1));
    }

    #[test]
    fn test_value() {
        let this = combine_intervals::combine_intervals(get_input());
        assert_eq!(this.get_value(1), 1);
        assert_eq!(this.get_value(2), 3);
        assert_eq!(this.get_value(3), 2);
        assert_eq!(this.get_value(4), 0);
    }

    #[test]
    fn test_contains_interval() {
        let this = combine_intervals::combine_intervals(get_input());
        assert!(this.contains_interval(Interval::new(1, 2, 1)));
        assert!(this.contains_interval(Interval::new(0, 2, 1)));
        assert!(this.contains_interval(Interval::new(0, 3, 1)));
        assert!(this.contains_interval(Interval::new(1, 2, 6)));
        assert!(!this.contains_interval(Interval::new(-1, 2, 1)));
    }

    // #[test]
    // fn test_value_interval_by_parts() {
    //     let this = combine_intervals::combine_intervals(get_input());
    //     let that: Vec<[i64; 3]> = vec!([1, 2, 1], [2, 3, 1]);
    //     let that = that.iter()
    //         .map(|x| Interval::new(x[0], x[1], x[2]))
    //         .collect();
    //     let that = IntervalCollection::from_vec(that);
    //     assert_eq!(this.get_value_of_interval_by_parts(Interval::new(-1, 4, 2)), that);
    // }
    //

    #[test]
    fn test_total_value() {
        let this = combine_intervals::combine_intervals(get_input());
        assert_eq!(this.total_value(), 6);
    }



    #[test]
    fn test_len() {
        let this = IntervalCollection::from_vec(get_input());
        assert_eq!(this.len(), 2);
        let this = combine_intervals::combine_intervals(get_input());
        assert_eq!(this.len(), 3);

    }
}
