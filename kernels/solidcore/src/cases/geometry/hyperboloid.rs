//! The solid inside x² + y² ≤ 16(1 + z²/36) between z = −6 and z = 6: the region of the xz plane
//! between the z axis, the planes z = ±6 and the hyperbola x²/16 − z²/36 = 1, turned a whole turn
//! about the z axis. The hyperbola's arc from (4√2, 0, −6) to (4√2, 0, 6) is the rational
//! quadratic it is: its ends, the point (2√2, 0, 0) where their tangents meet, weighted
//! cosh T = √2 for the arc x = 4 cosh t, z = 6 sinh t, −T ≤ t ≤ T, sinh T = 1.
use crate::bench::line;
use crate::shared::geometry::{axis, conic, turned};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let r = 4.0 * 2f64.sqrt();
    turned(vec![
        line([0.0, 0.0, -6.0], [r, 0.0, -6.0]),
        conic([r, 0.0, -6.0], [r / 2.0, 0.0, 0.0], 2f64.sqrt(), [r, 0.0, 6.0]),
        line([r, 0.0, 6.0], [0.0, 0.0, 6.0]),
        axis(6.0, -6.0),
    ])
}
