use std::ops::{Deref, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use thiserror::Error;

/// An `Interval` wraps the `std::ops::Range` from the stdlib and is defined by a start and end field
/// where end should be >= start.
#[derive(Default, Clone, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct Interval<N: Ord + Clone>(Range<N>);

#[derive(
    Error,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum IntervalError {
    #[error("an Interval must have a Range with a positive width")]
    InvalidRange,
}

impl<N: Ord + Clone> Interval<N> {
    /// Construct a new `Interval` from the given Range.
    /// Will return `Err` if end < start.
    pub fn new(r: Range<N>) -> Result<Interval<N>, IntervalError> {
        if r.end >= r.start {
            Ok(Interval(r))
        } else {
            Err(IntervalError::InvalidRange)
        }
    }
    pub fn contains(&self, other: Range<N>) -> bool {
        todo!()
    }
}

/// Convert a `Range` into an `Interval`. This conversion will panic if the `Range` has end < start
impl<N: Ord + Clone> From<Range<N>> for Interval<N> {
    fn from(r: Range<N>) -> Self {
        match Interval::new(r) {
            Ok(interval) => interval,
            Err(IntervalError::InvalidRange) => {
                panic!("Cannot convert negative width range to interval")
            }
        }
    }
}

/// Convert a reference to a `Range` to an interval by cloning. This conversion will panic if the
/// `Range` has end < start
impl<'a, N: Ord + Clone> From<&'a Range<N>> for Interval<N> {
    fn from(r: &Range<N>) -> Self {
        match Interval::new(r.clone()) {
            Ok(interval) => interval,
            Err(IntervalError::InvalidRange) => {
                panic!("Cannot convert negative width range to interval")
            }
        }
    }
}

/// Use the `Deref` operator to get a reference to `Range` wrapped by the `Interval` newtype.
impl<N: Ord + Clone> Deref for Interval<N> {
    type Target = Range<N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
