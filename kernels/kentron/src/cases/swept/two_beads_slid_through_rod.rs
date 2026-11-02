//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two beads slid past it: tori on one axis parallel to the y axis, the first centred at
//! (11.5, 0, −9) with radii 6 and 2, the second at (11.5, 1.5, −9) with radii 6 and 1.5, their
//! union slid 40 along the z axis from below the rod to above it. Everything the wheel passes
//! through, cut.
//!
//! kentron has no sweep of a solid (`sweep::extrude` takes faces, wires, edges, vertices and
//! compounds of them), so the wheel's sweep is built as elementary geometry gives it. The wheel
//! passes through a point when one of its beads does, so its sweep is the union of the beads'
//! sweeps; and each bead's is the bead at each end and the quarters of its face, split along the
//! circles where the slide runs along it, each slid (`shared::swept::slid_torus`). All are cut
//! from the rod in turn.
use crate::bench::{p, v};
use crate::shared::swept::{cut_each, slid_torus};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let slide = v(0.0, 0.0, 40.0);
    let mut pieces = slid_torus(m, p(11.5, 0.0, -9.0), 6.0, 2.0, slide)?;
    pieces.extend(slid_torus(m, p(11.5, 1.5, -9.0), 6.0, 1.5, slide)?);
    cut_each(m, rod, &pieces)
}
