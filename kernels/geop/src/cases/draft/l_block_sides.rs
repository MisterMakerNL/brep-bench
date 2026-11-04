//! An L-shaped block, the outline (0, 0), (30, 0), (30, 10), (10, 10), (10, 25), (0, 25)
//! extruded from z = 0 to z = 12, all six of its sides drafted 6° about the plane of its floor,
//! pulled along +z, so each leans into the material as it rises, the concave corner too. The
//! outline is sketched and extruded, and one draft of the six sides about the xy plane.
use crate::shared::blends::draft;
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.polygon(&[[0.0, 0.0], [30.0, 0.0], [30.0, 10.0], [10.0, 10.0], [10.0, 25.0], [0.0, 25.0]]);
    sketch(b, "outline", xy(), outline)?;
    extrude(b, "block", "outline", 12.0, Combine::NewBody)?;
    let sides = b.faces(&[
        [15.0, 0.0, 6.0],
        [30.0, 5.0, 6.0],
        [20.0, 10.0, 6.0],
        [10.0, 17.0, 6.0],
        [5.0, 25.0, 6.0],
        [0.0, 12.0, 6.0],
    ])?;
    b.step("taper", draft(sides, xy(), 6.0))
}
