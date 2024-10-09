use num_traits::{Num, One, Pow, ToPrimitive, Zero};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Default)]
pub struct IntFloat {
    base: isize,
    pow: isize,
}

impl IntFloat {
    pub fn new(base: isize, pow: isize) -> Self {
        IntFloat { base, pow }
    }

    pub fn from(float: f32, decimals: isize) -> Self {
        IntFloat {
            base: (float * isize::pow(10, decimals as u32) as f32).round() as isize,
            pow: decimals,
        }
    }

    pub fn set_base(mut self, new_base: isize) {
        self.base = new_base;
    }

    pub fn set_pow(mut self, new_pow: isize) {
        self.pow = new_pow;
    }

    pub fn print(self) -> String {
        if self.pow >= 1 {
            self.to_f32().unwrap().to_string()
        } else {
            self.to_i64().unwrap().to_string()
        }
    }
}

impl Debug for IntFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl Add<Self> for IntFloat {
    type Output = IntFloat;

    fn add(self, rhs: Self) -> IntFloat {
        if rhs.pow > self.pow {
            IntFloat {
                base: self.base * isize::pow(10, (rhs.pow - self.pow) as u32) + rhs.base,
                pow: rhs.pow,
            }
        } else {
            IntFloat {
                base: rhs.base * isize::pow(10, (self.pow - rhs.pow) as u32) + self.base,
                pow: self.pow,
            }
        }
    }
}

impl Add<IntFloat> for &mut IntFloat {
    type Output = IntFloat;

    fn add(self, rhs: IntFloat) -> IntFloat {
        *self + rhs
    }
}

impl AddAssign for IntFloat {
    fn add_assign(&mut self, rhs: Self) {
        let new = self.clone() + rhs;
        self.pow = new.pow;
        self.base = new.base;
    }
}

impl Zero for IntFloat {
    fn zero() -> Self {
        IntFloat { base: 0, pow: 0 }
    }

    fn is_zero(&self) -> bool {
        self.base == 0
    }
}

impl Mul<Self> for IntFloat {
    type Output = IntFloat;

    fn mul(self, rhs: Self) -> IntFloat {
        IntFloat {
            base: self.base * rhs.base,
            pow: self.pow + rhs.pow,
        }
    }
}

impl One for IntFloat {
    fn one() -> Self {
        IntFloat { base: 1, pow: 0 }
    }
}

impl Sub<Self> for IntFloat {
    type Output = IntFloat;

    fn sub(self, rhs: Self) -> IntFloat {
        let new_rhs = IntFloat {
            base: -rhs.base,
            pow: rhs.pow,
        };
        self.add(new_rhs)
    }
}

impl SubAssign for IntFloat {
    fn sub_assign(&mut self, rhs: Self) {
        let new = self.clone() - rhs;
        self.pow = new.pow;
        self.base = new.base;
    }
}

impl Div<Self> for IntFloat {
    type Output = IntFloat;

    fn div(self, rhs: Self) -> IntFloat {
        IntFloat {
            base: self.base / rhs.base,
            pow: self.pow - rhs.pow,
        }
    }
}

impl Rem<Self> for IntFloat {
    type Output = IntFloat;

    fn rem(self, rhs: Self) -> IntFloat {
        self - self.div(rhs).mul(rhs)
    }
}

impl Num for IntFloat {
    type FromStrRadixErr = ParseIntError;

    fn from_str_radix(str: &str, radix: u32) -> Result<IntFloat, ParseIntError> {
        let this_base = isize::from_str_radix(str, radix);
        let this_base = match this_base {
            Err(parse_int_error) => return Err(parse_int_error),
            Ok(num) => num,
        };
        Ok(IntFloat {
            base: this_base,
            pow: 0,
        })
    }
}

impl ToPrimitive for IntFloat {
    fn to_i64(&self) -> Option<i64> {
        Option::from((self.base as f64 * 10_f64.pow(-self.pow as f64)) as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Option::from((self.base as f64 * 10_f64.pow(-self.pow as f64)) as u64)
    }

    fn to_f32(&self) -> Option<f32> {
        Option::from(self.base as f32 * 10_f32.pow(-self.pow as f32))
    }

    fn to_f64(&self) -> Option<f64> {
        Option::from(self.base as f64 * 10_f64.pow(-self.pow as f64))
    }
}

impl std::iter::Sum for IntFloat {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut this = IntFloat::one();
        for i in iter {
            this += i;
        }
        this
    }
}
