use crate::BaseInterval;
use number_general::Number;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct IntervalCollection {
    intervals: Vec<BaseInterval>,
}

impl IntervalCollection {
    // Assumes ICs are always sorted by combine_intervals

    pub fn new() -> Self {
        IntervalCollection {
            intervals: Vec::new(),
        }
    }

    pub fn get_bounds(&self) -> (Number, Number) {
        // Todo: properly deal with empty collection
        (self.intervals[0].get_lb(), self.intervals.last().unwrap().get_ub())
    }

    pub fn get_lb(&self) -> Number {
        // Todo: properly deal with empty collection
        self.intervals[0].get_lb()
    }

    pub fn get_ub(&self) -> Number {
        // Todo: properly deal with empty collection
        self.intervals.last().unwrap().get_ub()
    }

    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    pub fn total_value(&self) -> Number {
        self.intervals.iter().map(|x| x.get_total_value()).sum::<Number>()
    }

    pub fn contains_num(&self, num: Number) -> bool {
        for interval in self.intervals.iter() {
            if interval.contains(num) {
                return true;
            }
        }
        false
    }

    pub fn get_value(&self, num: Number) -> Number {
        for interval in self.intervals.iter() {
            if interval.contains(num) {
                return interval.get_value();
            }
        }
        Number::from(0)
    }

    pub fn contains_interval(&self, interval: BaseInterval) -> bool {
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
                    BaseInterval::new(interval.get_ub(), to_check.get_ub(), to_check.get_value());
            }
        }
        false
    }

    pub fn get_value_of_interval_by_parts(&self, interval: BaseInterval) -> IntervalCollection {
        let mut values = Vec::new();
        let mut to_check = interval.clone();
        for interval in self.intervals.iter() {
            if interval.superset(to_check) {
                let new = BaseInterval::new(
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
                    BaseInterval::new(interval.get_ub(), to_check.get_ub(), to_check.get_value());
            }
        }
        IntervalCollection::from_vec(values)
    }

    pub fn get_partially_overlaps_interval(&self, other: &BaseInterval) -> bool {
        for interval in self.intervals.iter() {
            if interval.overlaps(*other) {
                return true;
            }
        }
        false
    }

    pub fn get_partially_overlaps(&self, other: IntervalCollection) -> bool {
        for interval in other.intervals.iter() {
            if self.get_partially_overlaps_interval(interval) {
                return true;
            }
        }
        false
    }

    pub fn from_vec(vec: Vec<BaseInterval>) -> Self {
        IntervalCollection { intervals: vec }
    }

    pub fn to_vec_owned(self) -> Vec<BaseInterval> {
        self.intervals
    }

    pub fn to_vec(self) -> Vec<BaseInterval> {
        self.intervals.clone()
    }


    pub fn to_vec_as_counter(&self) -> Vec<BaseInterval> {
        let mut new = Vec::new();
        if self.len() == 0 {
            return new;
        }
        let mut this_interval = self.intervals[0].val_to_count();
        for next_interval in self.intervals[1..].iter() {
            let next_count = next_interval.val_to_count();
            if this_interval.can_join(&next_count) {
                this_interval = this_interval.join(&next_count);
            } else {
                if this_interval.get_value() > Number::from(0) {
                    new.push(this_interval);
                }
                this_interval = next_count;
            }
        }
        if this_interval.get_value() > Number::from(0) {
            new.push(this_interval);
        }
        new
    }

    pub fn to_vec_as_set(&self) -> Vec<BaseInterval> {
        let mut new = Vec::new();
        if self.len() == 0 {
            return new;
        }
        let mut this_interval = self.intervals[0];
        for next_interval in self.intervals[1..].iter() {
            if this_interval.can_join_ign_value(next_interval) {
                this_interval = this_interval.join_ign_value(next_interval);
            } else {
                new.push(this_interval);
                this_interval = *next_interval;
            }
        }
        new.push(this_interval);
        new
    }
}
