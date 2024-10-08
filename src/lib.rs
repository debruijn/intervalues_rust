//! # Intervalues
//!
//! `intervalues` brings functionality to combine valued intervals together in an efficient manner.

mod base_interval;
mod combine_intervals;
mod interval;
mod interval_collection;

pub use crate::base_interval::BaseInterval;
pub use crate::combine_intervals::{combine_as_set, combine_intervals};
pub use crate::interval::Interval;
pub use crate::interval_collection::IntervalCollection;
