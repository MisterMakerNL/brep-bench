//! A drum of radius 8 about the z axis, its floor at z = 0, its top the saddle z = 10 + xy/8: a
//! cylinder reaching above the saddle, split by the bilinear B-spline patch through
//! (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and (10, 10, 22.5), and the part below kept.
use crate::shared::geometry::saddle_drum;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    saddle_drum()
}
