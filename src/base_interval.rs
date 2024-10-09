use num_traits::{Num, ToPrimitive};
use std::cmp::PartialOrd;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Interval<T: Num, U: Num> {
    lb: T,
    ub: T,
    val: U,
}


impl<T, U> Default for Interval<T, U>
where
    T: Num + PartialOrd + Clone,
    U: Num,
{
    fn default() -> Self {
        Interval {
            lb: T::zero(),
            ub: T::one(),
            val: U::one(),
        }
    }
}

impl<T, U> Interval<T, U>
where
    T: Num + PartialOrd + Clone,
    U: Num + PartialOrd,
{
    pub fn new(lb: T, ub: T, val: U) -> Self {
        if ub > lb {
            Interval { lb, ub, val }
        } else {
            Interval {
                lb: ub,
                ub: lb,
                val,
            }
        }
    }

    pub fn to_tuple(self) -> (T, T, U) {
        (self.lb, self.ub, self.val)
    }

    pub fn get_bounds(self) -> (T, T) {
        (self.lb, self.ub)
    }

    pub fn get_lb(self) -> T {
        self.lb
    }

    pub fn get_ub(self) -> T {
        self.ub
    }

    pub fn get_width(self) -> T {
        self.ub - self.lb
    }

    pub fn get_value(self) -> U {
        self.val
    }

    pub fn len(self) -> T {
        self.ub - self.lb
    }

    pub fn contains(self, num: T) -> bool {
        if (num >= self.lb) & (num <= self.ub) {
            true
        } else {
            false
        }
    }

    // TODO explore if T can be U here
    pub fn superset(self, other: Interval<T, U>) -> bool {
        if (other.ub <= self.ub) && (other.lb >= self.lb) {
            true
        } else {
            false
        }
    }

    pub fn subset(self, other: Interval<T, U>) -> bool {
        other.superset(self)
    }

    pub fn left_overlaps(&self, other: &Interval<T, U>) -> bool {
        if (self.lb <= other.lb) & (self.ub <= other.ub) {
            true
        } else {
            false
        }
    }

    pub fn right_overlaps(self, other: &Interval<T, U>) -> bool {
        other.left_overlaps(&self)
    }

    pub fn overlaps(self, other: Interval<T, U>) -> bool {
        self.left_overlaps(&other) || self.right_overlaps(&other)
    }

    pub fn can_join(self, other: &Interval<T, U>) -> bool {
        if ((self.ub == other.lb) || (other.ub == self.lb)) && (self.val == other.val) {
            true
        } else if (self.ub == other.ub) && (self.lb == other.lb) {
            true
        } else {
            false
        }
    }

    pub fn join(self, other: Interval<T, U>) -> Interval<T, U> {
        // Two options to enter this -> same range, or bordering range but same val
        // So test (and if so, return for) option 1, and then continue with option 2
        if (self.ub == other.ub) && (self.lb == other.lb) {
            return Interval::new(self.lb, self.ub, self.val + other.val);
        }

        // Option 2 from above
        let (lb, ub) = if self.lb < other.lb {
            (self.lb, other.ub)
        } else {
            (other.lb, self.ub)
        };
        Interval::new(lb, ub, self.val)
    }

    pub fn can_join_ign_value(self, other: &Interval<T, U>) -> bool {
        if (self.ub == other.lb) || (other.ub == self.lb) {
            true
        } else {
            false
        }
    }

    pub fn join_ign_value(self, other: Interval<T, U>) -> Interval<T, U> {
        let lb = if self.lb < other.lb {
            self.lb
        } else {
            other.lb
        };
        let ub = if self.ub > other.ub {
            self.ub
        } else {
            other.ub
        };
        Interval::new(lb, ub, U::one())
    }
}

impl<T> Interval<T, T>
where
    T: Num,
{
    pub fn get_total_value(self) -> T {
        (self.ub - self.lb) * self.val
    }
}

impl<T, U> Interval<T, U>
where
    T: Num + Clone + PartialOrd,
    U: Num + PartialOrd + ToPrimitive,
{
    pub fn val_to_count(self) -> Interval<T, usize> {
        // To test if this works
        if self.val >= U::one() {
            Interval::new(self.lb, self.ub, self.val.to_usize().unwrap())
        } else {
            Interval::new(self.lb, self.ub, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_int_interval() {
        let a = Interval::new(1, 2, 1);
        assert_eq!(a.len(), 1);
        assert_eq!(a.get_value(), 1)
    }

    #[test]
    fn test_create_float_interval() {
        let a = Interval::new(1.0, 4.0, 2.0);
        assert_eq!(a.len(), 3.0);
        assert_eq!(a.get_value(), 2.0);
        assert_eq!(a.get_total_value(), 6.0)
    }

    #[test]
    fn test_create_mixed_interval() {
        let a = Interval::new(1.0, 2.0, 1);
        assert_eq!(a.len(), 1.0);
        assert_eq!(a.get_value(), 1)
    }

    #[test]
    fn test_create_mixed_interval2() {
        let a = Interval::new(1, 2, 1.0);
        assert_eq!(a.len(), 1);
        assert_eq!(a.get_value(), 1.0)
    }

    #[test]
    fn test_val_to_count() {
        let a = Interval::new(1, 2, 1.5);
        assert_eq!(a.val_to_count().get_value(), 1)
    }

    #[test]
    fn test_val_to_count2() {
        let a = Interval::new(1, 2, 1);
        assert_eq!(a.val_to_count().get_value(), 1)
    }
}
