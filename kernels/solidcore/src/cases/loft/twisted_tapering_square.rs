//! A square column tapering as it twists, from z = 0 to z = 40: its section at height z the
//! square of side 30 − z/2 centred on the z axis, turned right-handed about the axis by 4.5° for
//! every unit of height, its corners on four conical spirals. solidcore's loft takes no guides,
//! and no curve of the kernel is a conical spiral, so the column is lofted through its sections
//! instead: the square of side 30 at z = 0 placed at every `STEP` of height by the turn, the
//! scale and the rise the description gives there (ops's sweep through stations), the kernel
//! skinning cubically through them, its ends capped.
use crate::bench::polygon;
use crate::Refusal;
use solidcore_ops::sweep::{sweep_stations, Affine, Ends};
use solidcore_ops::Body;

/// The height between consecutive sections.
const STEP: f64 = 0.5;

pub fn build() -> Result<Body, Refusal> {
    let foot = polygon(&[[15.0, 15.0, 0.0], [-15.0, 15.0, 0.0], [-15.0, -15.0, 0.0], [15.0, -15.0, 0.0]]);
    let count = (40.0 / STEP).round() as usize;
    let stations: Vec<Affine> = (0..=count)
        .map(|i| {
            let z = i as f64 * STEP;
            let (s, c) = (4.5 * z).to_radians().sin_cos();
            let scale = (30.0 - z / 2.0) / 30.0;
            Affine {
                m: [[scale * c, scale * s, 0.0], [-scale * s, scale * c, 0.0], [0.0, 0.0, 1.0]],
                t: [0.0, 0.0, z],
            }
        })
        .collect();
    Ok(sweep_stations(&foot, &stations, Ends::Capped)?)
}
