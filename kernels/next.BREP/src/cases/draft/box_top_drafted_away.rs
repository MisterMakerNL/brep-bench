//! The box from the origin to (30, 10, 10), its four sides drafted 30° for the pull direction +z
//! about the neutral plane z = 0, the floor: the long sides meet below the top, which the draft
//! consumes, leaving a hip roof. As the case states, the solid is the box cut by the four drafted
//! planes, each keeping the side that holds the box's middle: each plane through a bottom edge,
//! leaning 30° from +z into the box.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, split_solid_by_plane, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut solid = make_box_brep(p(0.0, 0.0, 0.0), 30.0, 10.0, 10.0)?;
    let (s, c) = 30f64.to_radians().sin_cos();
    // Each side's bottom edge by a point on it, and its outward normal (nx, ny); the drafted
    // plane's normal is that normal turned 30° up, toward +z.
    let sides = [
        (p(15.0, 0.0, 0.0), (0.0, -1.0)),
        (p(30.0, 5.0, 0.0), (1.0, 0.0)),
        (p(15.0, 10.0, 0.0), (0.0, 1.0)),
        (p(0.0, 5.0, 0.0), (-1.0, 0.0)),
    ];
    for (on_edge, (nx, ny)) in sides {
        let normal = p(nx * c, ny * c, s);
        // The piece below the plane, on the −normal side, holds the box's middle.
        solid = split_solid_by_plane(&solid, on_edge, normal)?.0;
    }
    Ok(solid)
}
