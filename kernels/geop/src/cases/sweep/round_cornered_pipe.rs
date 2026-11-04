//! A round pipe of radius 3 along two straight runs meeting at a right angle at the origin, from
//! (−30, 0, 0) along +x and on along +y to (0, 30, 0), the section carried round the corner by
//! turning it about the z axis: each run's disc extruded along it from the corner (the first on
//! the yz plane, 30 back along −x; the second on the xz plane, 30 along +y), and the wedge outside
//! the corner filled as the case composes it, by the half of the first run's end disc outside the
//! corner (y ≤ 0, drawn on the yz plane, its straight side on the z axis) turned a quarter turn
//! about the z axis to square to the second run — a quarter ball — all joined.
use crate::shared::{Drawing, extrude, revolve, sketch, sketch_axis, yz, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut first = Drawing::new();
    first.circle([0.0, 0.0], 3.0);
    sketch(b, "first_disc", yz(), first)?;
    extrude(b, "first_run", "first_disc", -30.0, Combine::NewBody)?;

    let mut second = Drawing::new();
    second.circle(zx_at(0.0, 0.0), 3.0);
    sketch(b, "second_disc", zx(), second)?;
    let pipe = b.solid()?;
    extrude(b, "second_run", "second_disc", 30.0, Combine::join(pipe))?;

    // On the yz plane, drawn in (y, z): the half disc y ≤ 0, up the z axis and back down round
    // through (−3, 0); turned about the z axis from −y towards +x, a turn geop's revolve counts
    // as −90° about this sketch's line.
    let mut half = Drawing::new();
    let top = half.point([0.0, 3.0]);
    let bottom = half.point([0.0, -3.0]);
    let on_axis = half.line(bottom, top);
    half.arc(top, bottom, 180.0);
    sketch(b, "half_disc", yz(), half)?;
    let pipe = b.solid()?;
    revolve(b, "corner", "half_disc", sketch_axis("half_disc", on_axis), -90.0, Combine::join(pipe))
}
