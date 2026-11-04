//! The solid inside the upper sheet of z²/9 − (x² + y²)/16 = 1, below z = 6: the region of the xz
//! plane between the z axis, the plane z = 6 and the hyperbola z²/9 − x²/16 = 1 from its vertex
//! (0, 0, 3) to (4√3, 0, 6), turned a whole turn about the z axis. The hyperbola's arc is the
//! rational quadratic it is: its ends, the point (4/√3, 0, 3) where their tangents meet, weighted
//! cosh(T/2) = √(3/2) for the arc x = 4 sinh t, z = 3 cosh t, 0 ≤ t ≤ T, cosh T = 2.
use crate::bench::line;
use crate::shared::geometry::{axis, conic, turned};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let rim = 4.0 * 3f64.sqrt();
    turned(vec![
        conic([0.0, 0.0, 3.0], [4.0 / 3f64.sqrt(), 0.0, 3.0], 1.5f64.sqrt(), [rim, 0.0, 6.0]),
        line([rim, 0.0, 6.0], [0.0, 0.0, 6.0]),
        axis(6.0, 3.0),
    ])
}
