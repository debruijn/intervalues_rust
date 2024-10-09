use num_traits::Num;
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct BaseInterval<T: Num> {
    lb: T,
    ub: T,
}

impl<T> Default for BaseInterval<T>
where
    T: Num + PartialOrd + Clone,
{
    fn default() -> Self {
        BaseInterval {
            lb: T::zero(),
            ub: T::one(),
        }
    }
}

impl<T> Debug for BaseInterval<T>
where
    T: Num + PartialOrd + Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T> Display for BaseInterval<T>
where
    T: Num + PartialOrd + Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T> BaseInterval<T>
where
    T: Num + PartialOrd + Clone + Display,
{
    pub fn new(lb: T, ub: T) -> Self {
        if ub > lb {
            BaseInterval { lb, ub }
        } else {
            BaseInterval { lb: ub, ub: lb }
        }
    }

    pub fn print(&self) -> String {
        format!("[{};{}]", self.lb, self.ub)
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.lb, self.ub)
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

    pub fn get_value(self) -> T {
        // For consistency
        T::one()
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

    pub fn superset(self, other: BaseInterval<T>) -> bool {
        if (other.ub <= self.ub) && (other.lb >= self.lb) {
            true
        } else {
            false
        }
    }

    pub fn subset(self, other: BaseInterval<T>) -> bool {
        other.superset(self)
    }

    pub fn left_overlaps(&self, other: &BaseInterval<T>) -> bool {
        if (self.lb <= other.lb) & (self.ub <= other.ub) & (other.lb <= self.ub) {
            true
        } else {
            false
        }
    }

    pub fn right_overlaps(self, other: &BaseInterval<T>) -> bool {
        other.left_overlaps(&self)
    }

    pub fn overlaps(self, other: BaseInterval<T>) -> bool {
        self.left_overlaps(&other) || self.right_overlaps(&other)
    }

    pub fn can_join(self, other: BaseInterval<T>) -> bool {
        // TODO: to test this more
        if self.overlaps(other) {
            true
        } else {
            false
        }
    }

    pub fn join(self, other: BaseInterval<T>) -> BaseInterval<T> {
        if (self.ub == other.ub) && (self.lb == other.lb) {
            return BaseInterval::new(self.lb, self.ub);
        }

        // Option 2 from above
        let (lb, ub) = if self.lb < other.lb {
            (self.lb, other.ub)
        } else {
            (other.lb, self.ub)
        };
        BaseInterval::new(lb, ub)
    }

    pub fn get_total_value(self) -> T {
        // For consistency
        self.get_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IntFloat;
    use num_traits::One;

    #[test]
    fn test_create_int_interval() {
        let a = BaseInterval::new(1, 2);
        assert_eq!(a.len(), 1);
        assert_eq!(a.get_value(), 1)
    }

    #[test]
    fn test_create_float_interval() {
        let a = BaseInterval::new(1.2, 4.2);
        assert_eq!(a.len(), 3.0);
        assert_eq!(a.get_value(), 1.0);
        assert_eq!(a.get_total_value(), 3.0)
    }

    #[test]
    fn test_create_intfloat_interval() {
        let a = BaseInterval::new(IntFloat::one(), IntFloat::from(2.0, 0));
        assert_eq!(a.len(), IntFloat::one());
        assert_eq!(a.get_value(), IntFloat::one());
        assert_eq!(a.get_total_value(), IntFloat::one())
    }

    #[test]
    fn test_bounds() {
        let a = BaseInterval::new(3, 7);
        assert_eq!(a.to_tuple(), (3, 7));
        assert_eq!(a.get_bounds(), (3, 7));
        assert_eq!(a.get_lb(), 3);
        assert_eq!(a.get_ub(), 7);
        assert_eq!(a.get_width(), 4);
    }

    #[test]
    fn test_total_value() {
        let a = BaseInterval::new(3, 7);
        assert_eq!(a.get_total_value(), 4);
        assert_eq!(a.get_value(), 1);
    }

    #[test]
    fn test_contains() {
        let a = BaseInterval::new(3, 7);
        assert!(a.contains(4));
        assert!(a.contains(3));
        assert!(a.contains(7));
        assert!(!a.contains(0));
    }

    #[test]
    fn test_superset_subset() {
        let a = BaseInterval::new(3, 7);
        let b = BaseInterval::new(4, 6);

        assert!(a.superset(b));
        assert!(b.subset(a));
        assert!(!a.subset(b));
        assert!(!b.superset(a));
    }

    #[test]
    fn test_overlaps() {
        let a = BaseInterval::new(3, 6);
        let b = BaseInterval::new(4, 7);

        assert!(a.left_overlaps(&b));
        assert!(b.right_overlaps(&a));
        assert!(!a.right_overlaps(&b));
        assert!(!b.left_overlaps(&a));
    }

    #[test]
    fn test_join() {
        let a = BaseInterval::new(0, 2);
        let b = BaseInterval::new(1, 4);
        let c = BaseInterval::new(3, 6);

        assert!(a.can_join(b));
        assert!(c.can_join(b));
        assert!(b.can_join(c));
        assert!(!a.can_join(c));

        let d = BaseInterval::new(0, 4);
        let e = BaseInterval::new(1, 6);

        assert_eq!(a.join(b), d);
        assert_eq!(c.join(b), e);
    }
}
