//! A cylinder of radius 10 about the z axis, from z = 0 to z = 20, its side split at the parting
//! line, its circle at z = 10, and each half drafted 3° away from it about the neutral plane
//! z = 10: the upper half with pull +z, the lower with pull −z, each leaning in as it leaves the
//! parting line. Two frustums of cones joined at their wide ends.
//!
//! The cylinder as its two mold halves, each the circle on the parting plane extruded 10 (one up,
//! one down), each half's side drafted away from the parting plane with cadrs's Draft, and the
//! halves united. (Splitting the whole cylinder's side at z = 10 with cadrs's Split of faces and
//! drafting each half in turn fails in the draft: "the moved faces don't close: face 0 loop 0:
//! p-curve gap after coedge 3".)
use crate::bench::{cylinder, face_at, p3, union, v3};
use crate::shared::draft::drafted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let parting = p3(0.0, 0.0, 10.0);
    let mut halves = vec![];
    for pull in [v3(0.0, 0.0, 1.0), v3(0.0, 0.0, -1.0)] {
        let half = cylinder(k, parting, pull, 10.0, 10.0)?;
        let side = face_at(k, half, parting + pull * 5.0 + v3(10.0, 0.0, 0.0))?;
        halves.push(drafted(k, half, vec![side], 3.0, pull, parting)?);
    }
    union(k, halves[0], halves[1])
}
