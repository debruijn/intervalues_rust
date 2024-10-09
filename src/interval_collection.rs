use crate::{BaseInterval, Interval};
use num_traits::{Num, ToPrimitive};
use safecast::CastInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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

    pub fn get_value_of_interval_by_parts(
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
            if interval.overlaps(*other) {
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

    pub fn to_vec(self) -> Vec<Interval<T, U>> {
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
