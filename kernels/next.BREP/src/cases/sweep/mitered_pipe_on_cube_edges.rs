//! A pipe of radius 2 swept along three edges of a cube of side 30, from (0, 0, 0) to (30, 0, 0), to
//! (30, 30, 0), to (30, 30, 30), mitered at its two corners. The kernel's sweep along a chain of
//! straight segments mitres each corner in the plane that bisects it; the disk stands square to
//! the path at its start, in the plane x = 0, and is carried by the path's own motion.
use crate::solids::p;
use crate::solids::sweep::circle;
use crate::Refusal;
use brep_kernel::{make_line, sweep_profile_along_chain, BrepSolid, SectionPlacement, SweepPath};

pub fn build() -> Result<BrepSolid, Refusal> {
    let points = [p(0.0, 0.0, 0.0), p(30.0, 0.0, 0.0), p(30.0, 30.0, 0.0), p(30.0, 30.0, 30.0)];
    let runs = (0..3).map(|i| make_line(points[i], points[i + 1])).collect::<Result<_, _>>()?;
    let path = SweepPath::new(runs, vec![])?;
    let disk = circle(points[0], p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0), 2.0)?;
    Ok(sweep_profile_along_chain(&disk, &path, 0.0, None, None, SectionPlacement::Rigid, "")?)
}
