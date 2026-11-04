//! A cylinder of radius 10 on the z axis from z = 0 to z = 20, its side split at the parting
//! line, the circle at z = 10, and each half drafted by 3° away from it about the neutral plane
//! z = 10: the upper half with pull +z, the lower with pull −z, each leaning inward as it goes
//! away from the parting line. Two frustums joined at their wide ends.
use crate::bench::{face_through, order};
use crate::shared::draft::draft_about_z;
use crate::Refusal;
use solidcore_ops::split::{split_faces, Tool};
use solidcore_ops::{make_cylinder, Body};
use solidcore_record::brep::Surface;

pub fn build() -> Result<Body, Refusal> {
    let cylinder = make_cylinder([0.0, 0.0, 0.0], 10.0, 20.0)?;
    // the parting line: the side split by the plane z = 10
    let side = face_through(&cylinder, [0.0, 10.0, 5.0])?;
    let parting = Tool {
        surface: Surface::Plane {
            origin: [0.0, 0.0, 10.0],
            u_axis: [1.0, 0.0, 0.0],
            v_axis: [0.0, 1.0, 0.0],
        },
        domain: [[-20.0, 20.0], [-20.0, 20.0]],
    };
    let (parted, _) = split_faces(&cylinder, &order(&cylinder)?, &[side], &[parting])?
        .ok_or("the plane z = 10 does not split the cylinder's side")?;
    draft_about_z(&parted, &[[0.0, 10.0, 15.0]], 10.0, [0.0, 0.0, 1.0], 3.0)?;
    draft_about_z(&parted, &[[0.0, 10.0, 5.0]], 10.0, [0.0, 0.0, -1.0], 3.0)?;
    Ok(parted)
}
