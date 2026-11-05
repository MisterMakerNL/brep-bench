//! A picture frame: the molding section swept around the closed rectangle with corners
//! (±30, ±20, 0), mitered at its four corners. The section, in (s, z) with s the distance outward
//! from the path, is the polygon (0, 2), (3, 2), (3, 0), (8, 0), (8, 6), (0, 6): a bar 8 × 6 outside
//! the path with a rabbet 3 × 2 out of its inner bottom edge. The kernel's sweep along a closed
//! chain of straight segments mitres every corner, the closing one included, and leaves no caps.
//! The section is drawn square to the path where it starts, at (−30, −20, 0) on the side y = −20
//! (the point (s, z) at (−30, −20 − s, z)), and carried by the path's own motion.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_line, sweep_profile_along_chain, BrepSolid, SectionPlacement, SweepPath};

pub fn build() -> Result<BrepSolid, Refusal> {
    let corners = [p(-30.0, -20.0, 0.0), p(30.0, -20.0, 0.0), p(30.0, 20.0, 0.0), p(-30.0, 20.0, 0.0)];
    let sides = (0..4).map(|i| make_line(corners[i], corners[(i + 1) % 4])).collect::<Result<_, _>>()?;
    let path = SweepPath::new(sides, vec![])?;
    let at = |s: f64, z: f64| p(-30.0, -20.0 - s, z);
    let section = polygon(&[
        at(0.0, 2.0), at(3.0, 2.0), at(3.0, 0.0), at(8.0, 0.0), at(8.0, 6.0), at(0.0, 6.0),
    ])?;
    Ok(sweep_profile_along_chain(&section, &path, 0.0, None, None, SectionPlacement::Rigid, "")?)
}
