//! What the solidcore implementations of the fillet cases share.
use crate::bench::PICK_TOLERANCE;
use crate::Refusal;
use solidcore_ops::{Body, Order};

/// The record index of the one vertex of `body` at `point`.
pub fn vertex_at(body: &Body, order: &Order, point: [f64; 3]) -> Result<usize, Refusal> {
    let mut found = Vec::new();
    for i in 0..order.vertices.len() {
        if body.vertex_distance(order, i, point)?.value < PICK_TOLERANCE {
            found.push(i);
        }
    }
    match found[..] {
        [v] => Ok(v),
        _ => Err(format!("{} vertices at {point:?}", found.len()).into()),
    }
}
