//! # Intervalues
//!
//! `intervalues` brings functionality to combine valued intervals together in an efficient manner.

mod base_interval;
mod combine_intervals;
// mod archive;

pub use crate::combine_intervals::{combine_intervals,
                                   combine_intervals_isize,
                                   combine_intervals_isize_no_val};
// pub use crate::archive::combine_intervals_general;
pub use crate::base_interval::BaseInterval;