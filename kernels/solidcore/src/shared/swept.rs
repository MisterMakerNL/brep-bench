//! What the solidcore implementations of the swept cases share.
//!
//! solidcore-ops sweeps curves, not bodies. The kernel itself sweeps a whole body, though: its
//! non-manifold sweeps (`SmApiNonManifoldSweep`, `SmApiNonManifoldRotationalSweep`) move every
//! face, edge and vertex of a B-rep along a line or about an axis, into the region outside it,
//! and with the merge on, join what they sweep with the body into one. [`slid`] and [`swung`]
//! call them on a body's record taken to the kernel and back, as `bench::faces_through` does.
use crate::bench::{arc, line};
use crate::Refusal;
use solidcore::kernel::Kernel;
use solidcore::sweep::api_brep::{
    non_manifold_rotational_sweep, non_manifold_sweep, RegionBrep, SweepSelection,
};
use solidcore::topology::brep::p5::make_manifold;
use solidcore::{Brep, Point3, Vec3};
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

/// A kernel configured as ops's session is: the kernel's defaults, with the upstream fixes the
/// runner turned on (`solidcore_ops::session::fixes`).
pub fn session_kernel() -> Kernel {
    let mut k = Kernel::default();
    for name in solidcore_ops::session::fixes() {
        if let Some(on) = k.fixes.flag_mut(name) {
            *on = true;
        }
    }
    k
}

/// `body` taken to the kernel, changed by `edit`, and brought back.
fn in_kernel(
    body: &Body,
    edit: impl FnOnce(&mut Kernel, &mut Brep) -> solidcore::Result<()>,
) -> Result<Body, Refusal> {
    let (record, _) = body.to_record()?;
    let mut k = session_kernel();
    let (mut brep, _) = solidcore_convert::brep::to_kernel(&mut k, &record, true)
        .map_err(|e| format!("the body's record does not convert: {e:?}"))?;
    edit(&mut k, &mut brep)?;
    brep.refresh_cached_topology();
    // the faces the sweep leaves between the body's material regions taken out
    make_manifold(&mut k, &mut brep, None, false)?;
    brep.refresh_cached_topology();
    let (record, _) = solidcore_convert::brep::to_record(&mut k, &mut brep)
        .map_err(|e| format!("the swept body's record was not made: {e:?}"))?;
    Ok(Body::from_record(&record, true)?.0)
}

/// Everything `body` passes through as it slides by `by` without turning: the kernel's sweep of
/// the whole body along `by` (`SmApiNonManifoldSweep`), stitched and merged.
pub fn slid(body: &Body, by: [f64; 3]) -> Result<Body, Refusal> {
    let d = Vec3::new(by[0], by[1], by[2]);
    let length = (by[0] * by[0] + by[1] * by[1] + by[2] * by[2]).sqrt();
    in_kernel(body, |k, b| {
        non_manifold_sweep(
            k,
            b,
            Some(RegionBrep::Swept),
            d,
            length,
            true,
            true,
            SweepSelection::default(),
        )
    })
}

/// The prism, from z = `z0` up `height`, on the stadium in that plane of the two discs of radius
/// `r` about (`x0`, `y`) and (`x1`, `y`) and the band between them: two lines and two half
/// circles.
pub fn stadium_prism(
    x0: f64,
    x1: f64,
    y: f64,
    r: f64,
    z0: f64,
    height: f64,
) -> solidcore_ops::Result<Body> {
    let up = [0.0, 0.0, 1.0];
    let outline = [
        line([x0, y - r, z0], [x1, y - r, z0]),
        arc([x1, y, z0], up, [1.0, 0.0, 0.0], r, -FRAC_PI_2, FRAC_PI_2),
        line([x1, y + r, z0], [x0, y + r, z0]),
        arc([x0, y, z0], up, [1.0, 0.0, 0.0], r, FRAC_PI_2, 3.0 * FRAC_PI_2),
    ];
    extrude(&outline, up, height, 0.0, DraftCorner::default(), true)
}

/// Everything `body` passes through as it turns `degrees` right-handed about the axis through
/// `base` along `axis`: the kernel's rotational sweep of the whole body
/// (`SmApiNonManifoldRotationalSweep`), stitched and merged.
pub fn swung(body: &Body, base: [f64; 3], axis: [f64; 3], degrees: f64) -> Result<Body, Refusal> {
    let p = Point3::new(base[0], base[1], base[2]);
    let a = Vec3::new(axis[0], axis[1], axis[2]);
    in_kernel(body, |k, b| {
        non_manifold_rotational_sweep(
            k,
            b,
            Some(RegionBrep::Swept),
            p,
            a,
            degrees,
            true,
            true,
            SweepSelection::default(),
        )
    })
}
