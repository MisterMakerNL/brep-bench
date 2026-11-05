//! A block 20 × 12 × 5 from (−10, −6, 0) to (10, 6, 5), its four vertical edges rounded to radius
//! 3 (the rounded rectangle extruded 5 up the z axis), and the rim of its top face, eight edges
//! running tangent into one another, rounded to radius 1 in one fillet around the whole chain.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_arc, make_line, BrepSolid};
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let (a, b, r) = (10.0, 6.0, 3.0);
    let outline = vec![
        make_line(p(-a + r, -b, 0.0), p(a - r, -b, 0.0))?,
        make_arc(p(a - r, -b + r, 0.0), x, y, r, -FRAC_PI_2, 0.0)?,
        make_line(p(a, -b + r, 0.0), p(a, b - r, 0.0))?,
        make_arc(p(a - r, b - r, 0.0), x, y, r, 0.0, FRAC_PI_2)?,
        make_line(p(a - r, b, 0.0), p(-a + r, b, 0.0))?,
        make_arc(p(-a + r, b - r, 0.0), x, y, r, FRAC_PI_2, PI)?,
        make_line(p(-a, b - r, 0.0), p(-a, -b + r, 0.0))?,
        make_arc(p(-a + r, -b + r, 0.0), x, y, r, PI, 1.5 * PI)?,
    ];
    let block = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 5.0)?;
    // the top rim's eight edges, each by its middle
    let h = 5.0;
    let c = r * std::f64::consts::FRAC_1_SQRT_2;
    let rim = [
        p(0.0, -b, h), p(a, 0.0, h), p(0.0, b, h), p(-a, 0.0, h),
        p(a - r + c, -b + r - c, h), p(a - r + c, b - r + c, h),
        p(-a + r - c, b - r + c, h), p(-a + r - c, -b + r - c, h),
    ];
    Ok(fillet_edges(&block, &rim, None, 1.0, false, None)?)
}
