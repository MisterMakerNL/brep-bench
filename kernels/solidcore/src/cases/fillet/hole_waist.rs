//! A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 through it on the z axis;
//! both rims of the hole, on the top and on the bottom, rounded to radius 1, half the plate's
//! thickness, so the rounds meet at the hole's waist and its wall vanishes.
use crate::bench::{cut, edges_through_each, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, make_cylinder, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 2.0])?;
    // the drill runs past both faces
    let drill = make_cylinder([0.0, 0.0, -1.0], 3.0, 4.0)?;
    let (body, order) = reloaded(&cut(plate, drill)?)?;
    let rims = edges_through_each(&body, &order, &[[0.0, 3.0, 2.0], [0.0, 3.0, 0.0]])?;
    blend(&body, Blend::Round, &rims, &[1.0], &[], Corners::Filled)?;
    Ok(body)
}
