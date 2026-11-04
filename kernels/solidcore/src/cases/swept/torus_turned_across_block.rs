//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from -1 to 1
//! and polar angle 0° to 60°, grooved by a cutter turned past it: the torus centred on (5, 0, 0)
//! with radii 2.5 and 2, its axis leaning from z towards +y by 25°, turned rigidly about the z
//! axis from -140° to +140°. Everything the cutter passes through, cut from the block.
//!
//! Refused. The turn is about one axis, and the kernel does sweep a whole body about an axis
//! (`SmApiNonManifoldRotationalSweep`, `shared::swept::swung`, which builds
//! `swept/ball-swung-groove`); but the leaning torus's wall folds over itself as it turns, its
//! sweep bounded by an envelope that is none of its own curves, and the kernel's sweep of it
//! fails after more than a minute (`boolean::merge::p2::make_faceuse: upstream status 1001`, in
//! the merge of what it swept). Ops has no other sweep of a solid, and the envelope would be
//! geometry computed outside the kernel.
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcore's sweep of a whole body about an axis (SmApiNonManifoldRotationalSweep) fails \
         on the leaning torus, whose turning wall folds over itself into an envelope that is none \
         of its own curves (boolean::merge::p2::make_faceuse: upstream status 1001, after more \
         than a minute); ops has no other sweep of a solid, and the envelope would be computed \
         outside the kernel"
        .into())
}
