//! An oblate spheroid, radius 6 at its equator and 3 to its poles on the z axis: the half ellipse
//! x²/36 + z²/9 ≤ 1, x ≥ 0, of the xz plane (the arc of brepkit's ellipse from the south pole over
//! (6, 0, 0) to the north, closed by the segment of the axis between them) turned a whole turn
//! about the z axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let (south, north) = (p(0.0, 0.0, -3.0), p(0.0, 0.0, 3.0));
    // about −y the ellipse's turn runs from x towards z: from the south pole out over (6, 0, 0)
    let half = plane_face(
        topo,
        vec![
            Seg::ellipse_arc(
                p(0.0, 0.0, 0.0),
                v(0.0, -1.0, 0.0),
                (6.0, 3.0),
                v(1.0, 0.0, 0.0),
                south,
                north,
            )?,
            Seg::Line(north, south),
        ],
    )?;
    Ok(revolve(
        topo,
        half,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        std::f64::consts::TAU,
    )?)
}
