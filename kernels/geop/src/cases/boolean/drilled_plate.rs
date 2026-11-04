//! A plate from the origin to (100, 20, 10), drilled through along z by nine holes of radius 2 at
//! y = 10 and x = 10, 20, … 90: a rectangle on the xy plane extruded 10 up, then, one hole after
//! another, a circle on the xy plane extruded from z = −1 to z = 11, past both faces, and cut
//! from the plate — each hole its own cut.
use crate::shared::booleans::extrude_between;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.rectangle([0.0, 0.0], [100.0, 20.0]);
    sketch(b, "outline", xy(), outline)?;
    extrude(b, "plate", "outline", 10.0, Combine::NewBody)?;
    let mut target = made("extrude", "plate");
    for i in 1..=9 {
        let mut bore = Drawing::new();
        bore.circle([10.0 * i as f64, 10.0], 2.0);
        let (bore_id, hole_id) = (format!("bore{i}"), format!("hole{i}"));
        sketch(b, &bore_id, xy(), bore)?;
        extrude_between(b, &hole_id, &bore_id, -1.0, 11.0, Combine::cut(target))?;
        target = made("extrude", &hole_id);
    }
    Ok(())
}
