//! A four-armed star handle: its outline on the xz plane — each arm's tip an arc of a circle of
//! radius 4 centred 33.1 out on an axis, each gap between arms an arc of a circle of radius 3
//! centred 15.9 out on a diagonal, joined by straight sides tangent to both, passing between
//! them — extruded 12 along y with a 2° draft, its walls leaning in away from the sketch.
//!
//! The outline is symmetric about the x axis, so it is drawn the same in the sketch's (x, −z) as
//! in (x, z). Each side's points of tangency are where the line between a tip circle and the
//! fillet circle beside it touches them: the side's normal `u` makes the angle `acos(7 / D)` with
//! the line between the centres, `D` apart, so that it touches the tip circle at `T + 4u` and the
//! fillet circle at `F − 3u`.
use crate::shared::{Drawing, sketch, zx};
use crate::{Bench, Refusal};
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let (tip, fillet) = (33.1, 15.9 / 2f64.sqrt());
    // The first arm's, along +x; the others are it turned about the origin.
    let (fx, fy) = (fillet - tip, fillet);
    let d = fx.hypot(fy);
    let alpha = fy.atan2(fx) - (7.0 / d).acos();
    let (ux, uy) = (alpha.cos(), alpha.sin());
    let upper_tip = [tip + 4.0 * ux, 4.0 * uy];
    let upper_fillet = [fillet - 3.0 * ux, fillet - 3.0 * uy];
    let turned = |[x, y]: [f64; 2], k: usize| {
        let a = (90.0 * k as f64).to_radians();
        [x * a.cos() - y * a.sin(), x * a.sin() + y * a.cos()]
    };
    // The fillet's other point of tangency is its first's mirror image in the diagonal.
    let tip_sweep = 2.0 * alpha.to_degrees();
    let fillet_sweep = 90.0 - 2.0 * alpha.to_degrees();

    let mut outline = Drawing::new();
    let mut corners = Vec::new();
    for k in 0..4 {
        let lower_tip = [upper_tip[0], -upper_tip[1]];
        let next_fillet = [upper_fillet[1], upper_fillet[0]];
        corners.push([
            outline.point(turned(lower_tip, k)),
            outline.point(turned(upper_tip, k)),
            outline.point(turned(upper_fillet, k)),
            outline.point(turned(next_fillet, k)),
        ]);
    }
    for k in 0..4 {
        let [a, b, c, d] = corners[k];
        outline.arc(a, b, tip_sweep);
        outline.line(b, c);
        outline.arc(c, d, fillet_sweep);
        outline.line(d, corners[(k + 1) % 4][0]);
    }
    sketch(b, "outline", zx(), outline)?;
    b.step(
        "handle",
        ExtrudeArgs {
            sketch: whole_sketch("outline"),
            extent: Extents::blind(12.0),
            output: Output::Solid(Combine::NewBody),
            draft: 2.0,
        },
    )
}
