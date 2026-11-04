//! A sheet, five faces of the cube from the origin to (10, 10, 10) — its floor and four walls,
//! open at the top — thickened by 1 out of the cube with its edges rounded: the gap at each of
//! the tray's eight edges closed by the quarter cylinder of radius 1 about it, and at each of the
//! floor's corners by the eighth ball of radius 1 about it. The sheet is the square's outline on
//! the xy plane extruded 10 as faces, the walls; the boundary surface filling their bottom edges,
//! the floor; the two knit into one open tray, whose normals point out of the cube. geop's
//! thicken meets the moved faces sharp, with no option to round them; so the eight outer edges
//! it leaves 1 out from the tray's are filleted to radius 1, whose rounds are those quarter
//! cylinders about the tray's edges, and where three meet, the ball about its corner.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::element;
use geop_ops_model::sketch3d::CurveSource;
use geop_ops_surface::{BoundarySurfaceArgs, KnitArgs};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "outline", xy(), outline)?;
    sheet_extrude(b, "walls", "outline", 10.0)?;
    let bottom = b.edges(&[[5.0, 0.0, 0.0], [10.0, 5.0, 0.0], [5.0, 10.0, 0.0], [0.0, 5.0, 0.0]])?;
    let edges = bottom.into_iter().map(|e| CurveSource::Edge(element(e))).collect();
    b.step("floor", BoundarySurfaceArgs { edges, tangent: Vec::new() })?;
    let faces = b.faces(&[
        [5.0, 0.0, 5.0], [10.0, 5.0, 5.0], [5.0, 10.0, 5.0], [0.0, 5.0, 5.0], [5.0, 5.0, 0.0],
    ])?;
    let faces = faces.into_iter().map(element).collect();
    b.step("tray", KnitArgs { faces, solid: false, gap: 0.0 })?;
    let floor = b.face([5.0, 5.0, 0.0])?;
    thicken(b, "box", &floor, 1.0, Side::Along)?;
    let outer = b.edges(&[
        [5.0, -1.0, -1.0], [11.0, 5.0, -1.0], [5.0, 11.0, -1.0], [-1.0, 5.0, -1.0],
        [-1.0, -1.0, 5.0], [11.0, -1.0, 5.0], [11.0, 11.0, 5.0], [-1.0, 11.0, 5.0],
    ])?;
    b.step("round", FilletArgs::constant(outer, 1.0))
}
