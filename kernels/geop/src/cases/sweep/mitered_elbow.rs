//! A square bar 10 × 10 swept along two straight runs, from the origin along +x to (40, 0, 0),
//! then turning 120° to the left on 40 to (20, 20√3, 0), mitred where they meet: the path
//! sketched on the xy plane as two lines, the square on the yz plane, centred on the path where it
//! starts, and swept along it — geop's sweep mitres two lines meeting at an angle, in the plane
//! bisecting the corner.
use crate::shared::paths::{path2d, sweep};
use crate::shared::{Drawing, sketch, xy, yz};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut path = Drawing::new();
    let p = [[0.0, 0.0], [40.0, 0.0], [20.0, 20.0 * 3f64.sqrt()]].map(|q| path.point(q));
    path.line(p[0], p[1]);
    path.line(p[1], p[2]);
    sketch(b, "path", xy(), path)?;

    let mut section = Drawing::new();
    section.rectangle([-5.0, -5.0], [5.0, 5.0]);
    sketch(b, "section", yz(), section)?;
    sweep(b, "bar", "section", path2d("path"), Combine::NewBody)
}
