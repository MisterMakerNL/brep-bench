//! Two blocks 10 high on the xy plane, one over x and y from 0 to 10, the other from 5 to 15:
//! the first less the second.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let first = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let second = make_box([5.0, 5.0, 0.0], [10.0, 10.0, 10.0])?;
    cut(first, second)
}
