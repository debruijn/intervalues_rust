use number_general::Number;


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct BaseInterval {
    lb: Number,
    ub: Number,
    val: Number
}

impl Default for BaseInterval {
    fn default() -> Self {
        BaseInterval{lb: Number::from(0), ub: Number::from(1), val: Number::from(1)}
    }
}

impl BaseInterval {
    pub fn new(lb: Number, ub: Number, val: Number) -> Self {
        BaseInterval{lb, ub, val}
    }

    pub fn to_vec(self) -> Vec<Number> {
        vec!(self.lb, self.ub, self.val)
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
            BaseInterval { lb: vec[0], ub: vec[1], val }
        } else {
            BaseInterval { lb: vec[1], ub: vec[0], val }
        }
    }

    // Next steps:
    // - Test in Python
    // - Add wrapper around Vec<BaseInterval> as result from combine
    // - Return as Counter, Set, Meter

}
