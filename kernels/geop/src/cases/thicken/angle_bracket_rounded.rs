//! A sheet of two flat faces meeting at a right angle along a crease — the floor, from the
//! origin to (10, 8, 0), and the wall, from the origin to (0, 8, 10) — thickened by 1 to the
//! outside of the crease (−z and −x), the gap at the crease closed by the quarter cylinder of
//! radius 1 about it. The sheet is the L of the xz plane, (10, 0) to the origin to (0, 10),
//! extruded 8 along +y as faces. geop's thicken meets the moved faces sharp, with no option to
//! round them; so the outer corner it leaves, along (−1, y, −1), is filleted to radius 1, whose
//! round is that quarter cylinder about the crease.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut l = Drawing::new();
    let [end, crease, top] =
        [(10.0, 0.0), (0.0, 0.0), (0.0, 10.0)].map(|(x, z)| l.point(zx_at(x, z)));
    l.line(end, crease);
    l.line(crease, top);
    sketch(b, "l", zx(), l)?;
    sheet_extrude(b, "sheet", "l", 8.0)?;
    let floor = b.face([5.0, 4.0, 0.0])?;
    thicken(b, "angle", &floor, 1.0, Side::Along)?;
    let corner = b.edge([-1.0, 4.0, -1.0])?;
    b.step("round", FilletArgs::constant(vec![corner], 1.0))
}
