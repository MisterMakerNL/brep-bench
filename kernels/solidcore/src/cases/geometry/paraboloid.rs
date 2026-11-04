//! A solid paraboloid x² + y² ≤ 8z, z ≤ 8: the region of the xz plane between the z axis, the
//! plane z = 8 and the parabola x² = 8z from its vertex to (8, 0, 8), turned a whole turn about
//! the z axis. The parabola's arc is the quadratic Bézier it is: its ends and the point (4, 0, 0)
//! where their tangents meet.
use crate::bench::line;
use crate::shared::geometry::{axis, conic, turned};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    turned(vec![
        conic([0.0, 0.0, 0.0], [4.0, 0.0, 0.0], 1.0, [8.0, 0.0, 8.0]),
        line([8.0, 0.0, 8.0], [0.0, 0.0, 8.0]),
        axis(8.0, 0.0),
    ])
}
