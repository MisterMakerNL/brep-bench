//! A sheet of two flat faces meeting at a right angle along a crease — the floor, from the
//! origin to (10, 8, 0), and the wall, from the origin to (0, 8, 10) — thickened by 1 to the
//! inside of the crease (+z and +x), the moved faces each trimmed where they meet. The sheet is
//! the L of the xz plane, (10, 0) to the origin to (0, 10), extruded 8 along +y as faces; its
//! normals point out of the crease, so it is thickened behind them.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, zx, zx_at};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut l = Drawing::new();
    let [end, crease, top] =
        [(10.0, 0.0), (0.0, 0.0), (0.0, 10.0)].map(|(x, z)| l.point(zx_at(x, z)));
    l.line(end, crease);
    l.line(crease, top);
    sketch(b, "l", zx(), l)?;
    sheet_extrude(b, "sheet", "l", 8.0)?;
    let floor = b.face([5.0, 4.0, 0.0])?;
    thicken(b, "angle", &floor, 1.0, Side::Against)
}
