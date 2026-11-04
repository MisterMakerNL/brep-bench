//! A disc of radius 20 on the z axis, from z = −5 to z = 0, with six holes of radius 2 through it
//! along z, their axes on the circle of radius 12, the first at (12, 0) and the others turned
//! by 60° each about the z axis: the disc a circle on the xy plane extruded 5 down; then, one
//! hole after another, a circle on the xy plane extruded from z = −6 to z = 1, past both faces,
//! and cut from the disc — each hole its own cut.
use crate::shared::booleans::extrude_between;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.circle([0.0, 0.0], 20.0);
    sketch(b, "outline", xy(), outline)?;
    extrude(b, "disc", "outline", -5.0, Combine::NewBody)?;
    let mut target = made("extrude", "disc");
    for i in 0..6 {
        let turn = (60.0 * i as f64).to_radians();
        let mut bore = Drawing::new();
        bore.circle([12.0 * turn.cos(), 12.0 * turn.sin()], 2.0);
        let (bore_id, hole_id) = (format!("bore{i}"), format!("hole{i}"));
        sketch(b, &bore_id, xy(), bore)?;
        extrude_between(b, &hole_id, &bore_id, -6.0, 1.0, Combine::cut(target))?;
        target = made("extrude", &hole_id);
    }
    Ok(())
}
