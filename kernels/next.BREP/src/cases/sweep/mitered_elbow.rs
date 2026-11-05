//! A 10 × 10 square bar swept along two straight runs, from (0, 0, 0) along +x to (40, 0, 0), then
//! turning 120° to the left and on 40 to (20, 20√3, 0), mitered at the corner. The kernel's sweep
//! along a chain of straight segments mitres every corner itself: each run is trimmed to the plane
//! through the corner that bisects it, and the two share the miter's face loop. The square stands
//! square to the path at its start, in the plane x = 0, and is carried by the path's own motion.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_line, sweep_profile_along_chain, BrepSolid, SectionPlacement, SweepPath};

pub fn build() -> Result<BrepSolid, Refusal> {
    let corner = p(40.0, 0.0, 0.0);
    let end = p(20.0, 20.0 * 3f64.sqrt(), 0.0);
    let path = SweepPath::new(
        vec![make_line(p(0.0, 0.0, 0.0), corner)?, make_line(corner, end)?],
        vec![],
    )?;
    let square = polygon(&[
        p(0.0, -5.0, -5.0), p(0.0, 5.0, -5.0), p(0.0, 5.0, 5.0), p(0.0, -5.0, 5.0),
    ])?;
    Ok(sweep_profile_along_chain(&square, &path, 0.0, None, None, SectionPlacement::Rigid, "")?)
}
