use crate::BaseInterval;
use num_traits::{Num, ToPrimitive};
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
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

impl<T, U> Debug for Interval<T, U>
where
    T: Num + PartialOrd + Clone + Display,
    U: Num + PartialOrd + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T, U> Display for Interval<T, U>
where
    T: Num + PartialOrd + Clone + Display,
    U: Num + PartialOrd + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T, U> Interval<T, U>
where
    T: Num + PartialOrd + Clone + Display,
    U: Num + PartialOrd + Display,
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

    pub fn print(&self) -> String {
        format!("[{};{}]x{}", self.lb, self.ub, self.val)
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
        if (self.lb <= other.lb) & (self.ub <= other.ub) & (other.lb <= self.ub) {
            true
        } else {
            false
        }
    }

    pub fn right_overlaps(self, other: &Interval<T, U>) -> bool {
        other.left_overlaps(&self)
    }

    pub fn overlaps(self, other: &Interval<T, U>) -> bool {
        self.left_overlaps(other) || self.right_overlaps(other)
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

    pub fn can_join_as_set(self, other: &Interval<T, U>) -> bool {
        if self.overlaps(other) {
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

    pub fn join_as_set(self, other: Interval<T, U>) -> BaseInterval<T> {
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
        BaseInterval::new(lb, ub)
    }

    pub fn to_base(self) -> BaseInterval<T> {
        BaseInterval::new(self.lb, self.ub)
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
    T: Num + Clone + PartialOrd + Display,
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
    use num_traits::One;
    use crate::IntFloat;
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
    fn test_create_intfloat_interval() {
        let a = Interval::new(IntFloat::one(), IntFloat::from(3.0, 0), IntFloat::from(3.0, 0));
        assert_eq!(a.len(), IntFloat::from(2.0, 0));
        assert_eq!(a.get_value(), IntFloat::from(3.0, 0));
        assert_eq!(a.get_total_value(), IntFloat::from(6.0, 0))
    }

    #[test]
    fn test_bounds() {
        let a = Interval::new(3, 7,  2);
        assert_eq!(a.to_tuple(), (3, 7, 2));
        assert_eq!(a.get_bounds(), (3, 7));
        assert_eq!(a.get_lb(), 3);
        assert_eq!(a.get_ub(), 7);
        assert_eq!(a.get_width(), 4);
    }

    #[test]
    fn test_total_value() {
        let a = Interval::new(3, 7, 2);
        assert_eq!(a.get_total_value(), 8);
        assert_eq!(a.get_value(), 2);
    }

    #[test]
    fn test_contains() {
        let a = Interval::new(3, 7, 2);
        assert!(a.contains(4));
        assert!(a.contains(3));
        assert!(a.contains(7));
        assert!(!a.contains(0));
    }

    #[test]
    fn test_superset_subset() {
        let a = Interval::new(3, 7, 2);
        let b = Interval::new(4, 6, 1);

        assert!(a.superset(b));
        assert!(b.subset(a));
        assert!(!a.subset(b));
        assert!(!b.superset(a));
    }

    #[test]
    fn test_overlaps() {
        let a = Interval::new(3, 6, 1);
        let b = Interval::new(4, 7, 2);

        assert!(a.left_overlaps(&b));
        assert!(b.right_overlaps(&a));
        assert!(!a.right_overlaps(&b));
        assert!(!b.left_overlaps(&a));
    }

    #[test]
    fn test_join() {
        let a = Interval::new(0, 2, 1);
        let b = Interval::new(2, 4, 2);
        let c = Interval::new(4, 6, 2);

        assert!(!a.can_join(&b));
        assert!(c.can_join(&b));
        assert!(b.can_join(&c));
        assert!(!a.can_join(&c));

        let d = Interval::new(0, 2, 2);
        let e = Interval::new(2,6, 2);

        assert_eq!(a.join(a), d);
        assert_eq!(c.join(b), e);
    }

    #[test]
    fn test_join_ign_value() {
        let a = Interval::new(0, 2, 2);
        let b = Interval::new(1, 4, 3);
        let c = Interval::new(3, 6, 6);

        assert!(a.can_join_as_set(&b));
        assert!(c.can_join_as_set(&b));
        assert!(b.can_join_as_set(&c));
        assert!(!a.can_join_as_set(&c));

        let d = BaseInterval::new(0, 4);
        let e = BaseInterval::new(1,6);
        let d2 = Interval::new(0, 4, 1);
        let e2 = Interval::new(1,6, 1);

        assert_eq!(a.join_as_set(b), d);
        assert_eq!(c.join_as_set(b), e);

        assert_eq!(a.join_ign_value(b), d2);
        assert_eq!(c.join_ign_value(b), e2);
    }


    #[test]
    fn test_val_to_count() {
        let a = Interval::new(0, 2, 3.5);
        let b = Interval::new(0, 2, 3);
        assert_eq!(a.val_to_count(), b);

        let c = Interval::new(0, 2, -3.5);
        let d = Interval::new(0, 2, 0);
        assert_eq!(c.val_to_count(), d);
    }

    #[test]
    fn test_to_base() {
        let a = Interval::new(0, 2, 3.5);
        let b = BaseInterval::new(0, 2);
        assert_eq!(a.to_base(), b)
    }

}
