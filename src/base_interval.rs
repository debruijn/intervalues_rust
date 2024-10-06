use number_general::{Int, Number, NumberInstance};
use safecast::CastFrom;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct BaseInterval {
    lb: Number,
    ub: Number,
    val: Number,
}

impl Default for BaseInterval {
    fn default() -> Self {
        BaseInterval {
            lb: Number::from(0),
            ub: Number::from(1),
            val: Number::from(1),
        }
    }
}

impl BaseInterval {
    pub fn new(lb: Number, ub: Number, val: Number) -> Self {
        BaseInterval { lb, ub, val }
    }

    pub fn to_vec(self) -> Vec<Number> {
        vec![self.lb, self.ub, self.val]
    }

    pub fn to_array(self) -> [Number; 3] {
        [self.lb, self.ub, self.val]
    }

    pub fn get_bounds(self) -> (Number, Number) {
        (self.lb, self.ub)
    }

    pub fn get_lb(self) -> Number {
        self.lb
    }

    pub fn get_ub(self) -> Number {
        self.ub
    }

    pub fn get_value(self) -> Number {
        self.val
    }

    pub fn len(self) -> Number {
        self.ub - self.lb
    }

    pub fn get_total_value(self) -> Number {
        self.get_value() * self.len()
    }

    pub fn contains(self, num: Number) -> bool {
        if (num >= self.lb) & (num <= self.ub) {
            true
        } else {
            false
        }
    }

    pub fn from_vec(vec: Vec<Number>) -> Self {
        // if vec.len() < 2 || vec.len() > 3 {
        //     return Err  # TODO add better error handling
        // };
        let val = if vec.len() == 2 {
            Number::from(1.0)
        } else {
            vec[2]
        };
        if vec[0] < vec[1] {
            BaseInterval {
                lb: vec[0],
                ub: vec[1],
                val,
            }
        } else {
            BaseInterval {
                lb: vec[1],
                ub: vec[0],
                val,
            }
        }
    }

    pub fn superset(self, other: BaseInterval) -> bool {
        if (other.ub <= self.ub) && (other.lb >= self.lb) {
            true
        } else {
            false
        }
    }

    pub fn subset(self, other: BaseInterval) -> bool {
        other.superset(self)
    }

    pub fn left_overlaps(self, other: BaseInterval) -> bool {
        if (self.lb <= other.lb) & (self.ub <= other.ub) {
            true
        } else {
            false
        }
    }

    pub fn right_overlaps(self, other: BaseInterval) -> bool {
        other.left_overlaps(self)
    }

    pub fn overlaps(self, other: BaseInterval) -> bool {
        self.left_overlaps(other) || self.right_overlaps(other)
    }

    pub fn can_join(self, other: &BaseInterval) -> bool {
        if ((self.ub == other.lb) || (other.ub == self.lb)) && (self.val == other.val) {
            true
        } else if (self.ub == other.ub) && (self.lb == other.lb) {
            true
        } else {
            false
        }
    }

    pub fn join(self, other: &BaseInterval) -> BaseInterval {
        // Two options to enter this -> same range, or bordering range but same val
        // So test (and if so, return for) option 1, and then continue with option 2
        if (self.ub == other.ub) && (self.lb == other.lb) {
            return BaseInterval::new(self.lb, self.ub, self.val + other.val);
        }

        // Option 2 from above
        let (lb, ub) = if self.lb < other.lb {
            (self.lb, other.ub)
        } else {
            (other.lb, self.ub)
        };
        BaseInterval::new(lb, ub, self.val)
    }

    pub fn val_to_count(self) -> BaseInterval {
        // To test if this works
        if self.val >= Number::from(1) {
            BaseInterval::new(self.lb, self.ub, Number::from(Int::cast_from(self.val)))
        } else {
            BaseInterval::new(self.lb, self.ub, Number::from(0))
        }
    }

    pub fn can_join_ign_value(self, other: &BaseInterval) -> bool {
        if (self.ub == other.lb) || (other.ub == self.lb) {
            true
        } else {
            false
        }
    }

    pub fn join_ign_value(self, other: &BaseInterval) -> BaseInterval {
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
        BaseInterval::new(lb, ub, Number::from(1))
    }
}
