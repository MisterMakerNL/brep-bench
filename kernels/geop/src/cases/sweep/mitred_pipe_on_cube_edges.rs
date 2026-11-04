//! A round pipe of radius 2 swept along three edges of a cube of side 30, from the origin to
//! (30, 0, 0), (30, 30, 0) and (30, 30, 30), mitred at its two corners: the path drawn in a 3-D
//! sketch as three lines, the circle on the yz plane, square to the path where it starts, and
//! swept along it — geop's sweep mitres two lines meeting at an angle, in the plane bisecting the
//! corner.
use crate::shared::paths::{at, path3d, sketch3d, sweep};
use crate::shared::{Drawing, sketch, yz};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;
use geop_ops_model::sketch3d::Sketch3d;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut route = Sketch3d::new();
    let p = [[0.0, 0.0, 0.0], [30.0, 0.0, 0.0], [30.0, 30.0, 0.0], [30.0, 30.0, 30.0]]
        .map(|q| route.add_point(at(q)));
    for w in p.windows(2) {
        route.add_line(w[0], w[1]);
    }
    sketch3d(b, "edges", route)?;

    let mut section = Drawing::new();
    section.circle([0.0, 0.0], 2.0);
    sketch(b, "section", yz(), section)?;
    sweep(b, "pipe", "section", path3d("edges"), Combine::NewBody)
}
