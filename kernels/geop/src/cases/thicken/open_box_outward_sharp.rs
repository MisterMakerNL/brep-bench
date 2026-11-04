//! A sheet, five faces of the cube from the origin to (10, 10, 10) — its floor and four walls,
//! open at the top — thickened by 1 out of the cube, the moved faces extended until they meet:
//! the box from (−1, −1, −1) to (11, 11, 10) less the cube. The sheet is the square's outline on
//! the xy plane extruded 10 as faces, the walls; the boundary surface filling their bottom edges,
//! the floor; the two knit into one open tray, whose normals point out of the cube: thickened
//! along them, geop's thicken meeting the moved faces sharp.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
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
    thicken(b, "box", &floor, 1.0, Side::Along)
}
