//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel
//! screwed along it: the torus of radii 6 and 2 centred on (11.5, 0, 0), its axis leaning by the
//! helix angle, carried by the screw about the z axis rising 30 a turn from a turn of -108° to
//! +348°. Everything the wheel passes through, cut from the rod.
//!
//! Refused. solidcore's screw sweep (`screw_sweep`) sweeps a profile's curves, not a body, and the
//! kernel sweeps a whole body only along a line or about an axis (`SmApiNonManifoldSweep`,
//! `SmApiNonManifoldRotationalSweep`). The wheel's sweep under the screw is bounded by the
//! envelope of its torus, none of its own curves: a profile for the screw sweep would be that
//! envelope's section, geometry computed outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore's screw sweep takes a profile's curves, not a body, and it sweeps a whole body \
         only along a line or about an axis; the wheel's sweep under the screw is bounded by the \
         torus's envelope, whose section would be computed outside the kernel"
        .into())
}
