//! A hollow square duct narrowing as it bends a quarter turn round an arc of radius 30 about the z
//! axis, from (30, 0, 0) to (0, 30, 0): its outer half-width running evenly from 9 to 6, its wall
//! 2 thick all round, so the hole's half-width runs from 7 to 4.
//!
//! geop's sweep scales the whole profile by one factor, growing evenly with the length travelled,
//! about the path's point in it; a hollow section keeping its wall's thickness scales its outside
//! and its hole by different factors. So the duct is the solid less its cavity: the outer square
//! (18 × 18, centred on the arc on the xz plane) swept along the arc scaled to 6/9, and the hole
//! (14 × 14) swept along it scaled to 4/7 and cut from it.
use crate::shared::paths::path2d;
use crate::shared::{Drawing, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_boolean::BooleanArgs;
use geop_ops_extrude_revolve::{Output, SweepArgs};
use geop_lib_sweep::Orientation;
use geop_ops_model::sketch::whole_sketch;
use geop_ops_model::{BooleanOp, Combine, solid};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut arc = Drawing::new();
    let (start, end) = (arc.point([30.0, 0.0]), arc.point([0.0, 30.0]));
    arc.arc(start, end, 90.0);
    sketch(b, "centreline", xy(), arc)?;

    for (id, half, end_half) in [("outside", 9.0, 6.0), ("hole", 7.0, 4.0)] {
        let section = format!("{id}_section");
        let mut square = Drawing::new();
        square.rectangle(zx_at(30.0 - half, -half), zx_at(30.0 + half, half));
        sketch(b, &section, zx(), square)?;
        b.step(
            id,
            SweepArgs {
                profile: whole_sketch(&section),
                path: Some(path2d("centreline")),
                orientation: Orientation::FollowPath,
                twist: 0.0,
                end_scale: end_half / half,
                rails: Vec::new(),
                output: Output::Solid(Combine::NewBody),
            },
        )?;
    }
    b.step(
        "duct",
        BooleanArgs { a: solid("sweep(outside)"), b: solid("sweep(hole)"), op: BooleanOp::Difference },
    )
}
