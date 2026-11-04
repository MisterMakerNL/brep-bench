//! The parabolic cylinder z = x²/8 for -4 <= x <= 4 and 0 <= y <= 10 as a B-spline surface, of
//! degree 2 in u and 1 in v, thickened by 1 to its concave side, toward the focus.
use solidcore_ops::sweep::{loft, Curve};
use solidcore_ops::{thicken, Body, Result};

pub fn build() -> Result<Body> {
    // the parabola's quadratic Bézier at y, with control points (-4, y, 2), (0, y, -2), (4, y, 2)
    let parabola = |y: f64| Curve::Nurbs {
        degree: 2,
        points: vec![[-4.0, y, 2.0, 1.0], [0.0, y, -2.0, 1.0], [4.0, y, 2.0, 1.0]],
        knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    };
    // skinned linearly from y = 0 to y = 10, uncapped: the B-spline sheet, degree 1 across
    let sheet = loft(&[vec![parabola(0.0)], vec![parabola(10.0)]], 1, false)?;
    // its normals face away from the focus: toward it is against them
    thicken(&sheet, -1.0, false)?;
    Ok(sheet)
}
