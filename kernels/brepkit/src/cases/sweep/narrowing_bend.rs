//! A hollow square duct narrowing round a quarter-turn bend of radius 30 about the z axis: the
//! outside, the square of half-width 9 standing in the xz plane centred at (30, 0, 0), swept
//! round the arc by brepkit's sweep with a scale law that takes its half-width linearly in the
//! angle turned from 9 to 6; less the hole, the square of half-width 7 swept likewise to 4. The
//! sections scale about the arc, on rotation-minimizing frames that on a plane arc keep them in
//! the plane through the z axis. The law is given in the angle turned, read from the arc's point
//! at each of its parameters (the arc's NURBS parameter is not its angle).
use std::f64::consts::FRAC_PI_2;

use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_operations::sweep::{sweep_with_options, SweepOptions};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{arc, polygon};
use crate::solids::{cut, p, v};
use crate::Refusal;

/// The square of half-width `start` swept round `path`, its half-width running linearly in the
/// angle turned to `end` at the quarter turn.
fn tapered(
    topo: &mut Topology,
    path: &NurbsCurve,
    start: f64,
    end: f64,
) -> Result<SolidId, Refusal> {
    let h = start;
    let section = polygon(
        topo,
        &[
            p(30.0 - h, 0.0, -h),
            p(30.0 + h, 0.0, -h),
            p(30.0 + h, 0.0, h),
            p(30.0 - h, 0.0, h),
        ],
    )?;
    let (t0, t1) = path.domain();
    let along = path.clone();
    let law = move |t: f64| {
        let at = along.evaluate(t0 + (t1 - t0) * t);
        let turned = at.y().atan2(at.x()) / FRAC_PI_2;
        (start + (end - start) * turned) / start
    };
    let options = SweepOptions {
        scale_law: Some(Box::new(law)),
        ..Default::default()
    };
    Ok(sweep_with_options(topo, section, path, &options)?)
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let path = arc(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        p(30.0, 0.0, 0.0),
        FRAC_PI_2,
    )?;
    let outside = tapered(topo, &path, 9.0, 6.0)?;
    let hole = tapered(topo, &path, 7.0, 4.0)?;
    Ok(cut(topo, outside, hole)?)
}
