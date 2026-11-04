//! A round mounting flange on the z axis: the plate's outline on the xy plane — a circle of
//! radius 32, the bore's circle of radius 8 and six bolt circles of radius 3 on a circle of
//! radius 24, one every 60°, as holes in it — extruded 6 up; then the hub's ring, radii 15 and 8,
//! on a plane 6 up, extruded 10 more and joined to it.
use crate::shared::paths::offset_plane;
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut plate = Drawing::new();
    plate.circle([0.0, 0.0], 32.0);
    plate.circle([0.0, 0.0], 8.0);
    for k in 0..6 {
        let angle = (60.0 * k as f64).to_radians();
        plate.circle([24.0 * angle.cos(), 24.0 * angle.sin()], 3.0);
    }
    sketch(b, "plate_outline", xy(), plate)?;
    extrude(b, "plate", "plate_outline", 6.0, Combine::NewBody)?;

    let top = offset_plane(b, "top", FrameAxis::Z, 6.0)?;
    let mut hub = Drawing::new();
    hub.circle([0.0, 0.0], 15.0);
    hub.circle([0.0, 0.0], 8.0);
    sketch(b, "hub_outline", top, hub)?;
    let plate = b.solid()?;
    extrude(b, "hub", "hub_outline", 10.0, Combine::join(plate))
}
