//! What the solidcore implementations of the nurbs cases share (and the geometry, loft and spline
//! cases that build a solid from faces): a solid sewn from its faces, each a surface given as data
//! (a B-spline or NURBS surface's net and knots, as a description gives them) or a flat face
//! bounded by curves.
//!
//! solidcore-ops has no operation that makes a face on a surface given as data, so the faces are
//! made in a kernel body: a surface's face by `SmBrep::CreateFaceFromSurface` over the surface's
//! whole parameter box (`sheet::face_on`), a flat face by `SmBrep::CreatePlanarFaceWith3DCurves`.
//! The body goes to solidcore-ops through its record ([`Body::from_record`]), where ops sews the
//! faces into a solid ([`stitch`]).
//!
//! Also: a solid split by a surface given as data, the part holding a point kept; the vaulted
//! block of the vault cases; the Bézier dome the dome cases cut; and a periodic B-spline as a
//! profile curve, clamped by the kernel.
use crate::bench::line;
use crate::Refusal;
use solidcore::curve::curve::Curve as KCurve;
use solidcore::curve::from_parameters::{from_parameters as curve_from_parameters, record_of};
use solidcore::curve::line::Line;
use solidcore::geom::tol::USE_DEFAULT;
use solidcore::kernel::Kernel;
use solidcore::surface::from_parameters::from_parameters as surface_from_parameters;
use solidcore::sweep::segments::build_faces;
use solidcore::topology::brep::p3::new_brep;
use solidcore::topology::brep::p8::create_planar_face_with_3d_curves;
use solidcore::topology::brep::sheet::face_on;
use solidcore::Point3;
use solidcore_ops::split::{split_body, Tool};
use solidcore_ops::stitch::stitch;
use solidcore_ops::sweep::{extrude, Curve as OpsCurve, DraftCorner};
use solidcore_ops::{Body, PointClass};
use solidcore_record::brep::{Curve, Surface};

/// The tolerance faces are sewn at: their edges are the same curves, given by the same numbers.
pub const SEW: f64 = 1e-6;

/// One face of a solid built from its faces.
#[derive(Clone, Debug)]
pub enum Face {
    /// The face over the whole parameter box of a B-spline or NURBS surface.
    Surface(Surface),
    /// The flat face bounded by one closed loop of curves (lines, B-splines and NURBS, each
    /// over its whole knot interval).
    Flat(Vec<Curve>),
}

/// A B-spline surface: `points[i][j]`, i along u, on the knot vectors given.
pub fn spline(u_knots: &[f64], v_knots: &[f64], points: Vec<Vec<[f64; 3]>>) -> Surface {
    Surface::Spline { u_knots: u_knots.to_vec(), v_knots: v_knots.to_vec(), points }
}

/// A NURBS surface from its control points and weights `[x, y, z, w]`, i along u.
pub fn rational_spline(u_knots: &[f64], v_knots: &[f64], points: &[Vec<[f64; 4]>]) -> Surface {
    let homogeneous =
        points.iter().map(|row| row.iter().map(|&p| homogeneous(p)).collect()).collect();
    Surface::RationalSpline { u_knots: u_knots.to_vec(), v_knots: v_knots.to_vec(), homogeneous }
}

/// A NURBS curve from its control points and weights `[x, y, z, w]`.
pub fn rational_curve(knots: &[f64], points: &[[f64; 4]]) -> Curve {
    Curve::RationalSpline { knots: knots.to_vec(), homogeneous: points.iter().map(|&p| homogeneous(p)).collect() }
}

/// A B-spline curve.
pub fn spline_curve(knots: &[f64], points: &[[f64; 3]]) -> Curve {
    Curve::Spline { knots: knots.to_vec(), points: points.to_vec() }
}

/// The segment from `a` to `b`.
pub fn segment(a: [f64; 3], b: [f64; 3]) -> Curve {
    Curve::Line { start: a, end: b }
}

/// The closed polygon through `points`, in order.
pub fn polygon(points: &[[f64; 3]]) -> Vec<Curve> {
    (0..points.len()).map(|i| segment(points[i], points[(i + 1) % points.len()])).collect()
}

/// The knots of a Bézier curve of `degree`: `degree + 1` zeros and as many ones.
pub fn bezier_knots(degree: usize) -> Vec<f64> {
    let mut k = vec![0.0; degree + 1];
    k.extend(vec![1.0; degree + 1]);
    k
}

/// A control point and its weight as the record holds it, `[w x, w y, w z, w]`.
fn homogeneous([x, y, z, w]: [f64; 4]) -> [f64; 4] {
    [x * w, y * w, z * w, w]
}

/// A record curve as the kernel's bounded curve: a line as its segment, a spline over its knots.
fn kernel_curve(k: &Kernel, c: &Curve) -> Result<KCurve, Refusal> {
    Ok(match c {
        Curve::Line { start, end } => KCurve::from(Line::create_line_segment(
            3,
            Point3::new(start[0], start[1], start[2]),
            Point3::new(end[0], end[1], end[2]),
            None,
        )?),
        Curve::Spline { .. } | Curve::RationalSpline { .. } => {
            curve_from_parameters(k, &solidcore_convert::curve(c), [0.0, 0.0])?.curve
        }
        _ => return Err("a flat face's curve is a line or a spline here".into()),
    })
}

/// The faces, each its own, in one body: a sheet of faces not yet sewn.
pub fn sheet(faces: &[Face]) -> Result<Body, Refusal> {
    let mut k = Kernel::default();
    let mut b = new_brep(&mut k, USE_DEFAULT, USE_DEFAULT)?;
    for face in faces {
        match face {
            Face::Surface(s) => {
                let (Surface::Spline { u_knots, v_knots, .. }
                | Surface::RationalSpline { u_knots, v_knots, .. }) = s
                else {
                    return Err("a face's surface is a B-spline or NURBS here".into());
                };
                let ends = |k: &[f64]| [k[0], k[k.len() - 1]];
                let (surface, domain) = surface_from_parameters(
                    &k,
                    &solidcore_convert::surface(s),
                    [ends(u_knots), ends(v_knots)],
                )?;
                face_on(&mut k, &mut b, surface, &domain)?;
            }
            Face::Flat(loop_) => {
                let curves = loop_.iter().map(|c| kernel_curve(&k, c)).collect::<Result<Vec<_>, _>>()?;
                let region = b.infinite_region();
                build_faces(&mut b, |b, fenv| {
                    create_planar_face_with_3d_curves(&mut k, b, region, &curves, SEW, fenv)
                })?
                .ok_or("a flat face's curves bound no face")?;
            }
        }
        b.refresh_cached_topology();
    }
    let (record, _) = solidcore_convert::brep::to_record(&mut k, &mut b)
        .map_err(|e| format!("the faces make no record: {e}"))?;
    Ok(Body::from_record(&record, true)?.0)
}

/// The solid the faces bound, sewn by ops along the edges they share ([`stitch`], at [`SEW`]).
pub fn sewn(faces: &[Face]) -> Result<Body, Refusal> {
    let (body, closed) = stitch(sheet(faces)?, SEW)?;
    if !closed {
        return Err("the faces sewn do not close into a solid".into());
    }
    Ok(body)
}

/// The block under a B-spline patch that is the graph of a height over the rectangle of its
/// corner control points: control point (i, j) is `(xs[i], ys[j], heights[i][j])`, on the knot
/// vectors `u_knots` (along x) and `v_knots` (along y). Its faces: the patch, the flat floor at
/// z = 0, and the four upright sides, each bounded by the floor's edge, two upright lines and the
/// curve the patch's edge draws in the side's plane (the B-spline of that edge's control points).
pub fn graph_block(
    xs: &[f64],
    ys: &[f64],
    heights: &[Vec<f64>],
    u_knots: &[f64],
    v_knots: &[f64],
) -> Result<Body, Refusal> {
    let (n, m) = (xs.len() - 1, ys.len() - 1);
    let net: Vec<Vec<[f64; 3]>> =
        (0..=n).map(|i| (0..=m).map(|j| [xs[i], ys[j], heights[i][j]]).collect()).collect();
    let floor = |i: usize, j: usize| [xs[i], ys[j], 0.0];
    let side = |edge: Vec<[f64; 3]>, knots: &[f64], a: [f64; 3], b: [f64; 3]| {
        let (first, last) = (edge[0], edge[edge.len() - 1]);
        Face::Flat(vec![segment(a, b), segment(b, last), spline_curve(knots, &edge), segment(first, a)])
    };
    let faces = vec![
        Face::Surface(spline(u_knots, v_knots, net.clone())),
        Face::Flat(polygon(&[floor(0, 0), floor(n, 0), floor(n, m), floor(0, m)])),
        side(net[0].clone(), v_knots, floor(0, 0), floor(0, m)),
        side(net[n].clone(), v_knots, floor(n, 0), floor(n, m)),
        side(net.iter().map(|r| r[0]).collect(), u_knots, floor(0, 0), floor(n, 0)),
        side(net.iter().map(|r| r[m]).collect(), u_knots, floor(0, m), floor(n, m)),
    ];
    sewn(&faces)
}

/// The closed, periodic (unclamped) uniform B-spline of `degree` on the control polygon
/// `polygon` (closing back on its first point), as a profile curve. The kernel holds clamped
/// B-splines only, so the periodic curve is given it as the IGES form of one (its first `degree`
/// points repeated at its end, its knots 0, 1, 2, … each once, its domain from knot `degree` to
/// the one past the last point's span), which the kernel clamps to that domain (its canonical
/// constructor, `N_Iges126Crv`); the clamped curve is the same curve, read back as ops's NURBS.
pub fn periodic(degree: usize, polygon: &[[f64; 3]]) -> Result<OpsCurve, Refusal> {
    let points: Vec<[f64; 3]> = (0..polygon.len() + degree).map(|i| polygon[i % polygon.len()]).collect();
    let knots: Vec<f64> = (0..points.len() + degree + 1).map(|i| i as f64).collect();
    let domain = [knots[degree], knots[points.len()]];
    let k = Kernel::default();
    let made = curve_from_parameters(&k, &solidcore_convert::curve(&spline_curve(&knots, &points)), domain)?;
    let clamped = solidcore_convert::record_curve(record_of(&k, &made.curve, &made.interval)?.curve);
    match clamped {
        Curve::Spline { knots, points } => Ok(OpsCurve::Nurbs {
            degree,
            points: points.iter().map(|p| [p[0], p[1], p[2], 1.0]).collect(),
            knots,
        }),
        other => Err(format!("the kernel's clamped curve is no B-spline: {other:?}").into()),
    }
}

/// The part of `body` split by `tool` ([`split_body`]) that holds the point `keep`.
pub fn split_keeping(body: &Body, tool: &Tool, keep: [f64; 3]) -> Result<Body, Refusal> {
    let (back, front) = split_body(body, tool)?.ok_or("the split leaves a side empty")?;
    for part in [back, front] {
        if part.classify_point(keep)? == PointClass::Inside {
            return Ok(part);
        }
    }
    Err(format!("neither part of the split holds {keep:?}").into())
}

/// The block of `nurbs/groin-vault` and `nurbs/vault-tangent-plane`: on the square
/// −12 ≤ x, y ≤ 12, its floor at z = 0, its top the rational B-spline of degree 1 along x and 2
/// along y through (x, −12, 10), (x, 0, 26) and (x, 12, 10), weighted 1, 3/5 and 1. Its section
/// in the plane x = −12, the floor's edge, the two upright sides' and the rational quadratic arc
/// of those control points, extruded 24 along x: the arc sweeps that surface.
pub fn vault() -> Result<Body, Refusal> {
    let x = -12.0;
    let arch = OpsCurve::Nurbs {
        degree: 2,
        points: vec![[x, 12.0, 10.0, 1.0], [x, 0.0, 26.0, 0.6], [x, -12.0, 10.0, 1.0]],
        knots: bezier_knots(2),
    };
    let section = vec![
        line([x, -12.0, 0.0], [x, 12.0, 0.0]),
        line([x, 12.0, 0.0], [x, 12.0, 10.0]),
        arch,
        line([x, -12.0, 10.0], [x, -12.0, 0.0]),
    ];
    Ok(extrude(&section, [1.0, 0.0, 0.0], 24.0, 0.0, DraftCorner::Curvature, true)?)
}

/// The block under the bicubic Bézier patch of `nurbs/bezier-dome` (control points
/// (20i/3, 20j/3, zᵢⱼ)), which `nurbs/drilled-dome` and `nurbs/sliced-dome` cut.
pub fn bezier_dome() -> Result<Body, Refusal> {
    const HEIGHTS: [[f64; 4]; 4] =
        [[4.0, 6.0, 7.0, 5.0], [6.0, 14.0, 12.0, 7.0], [5.0, 13.0, 16.0, 8.0], [3.0, 6.0, 8.0, 6.0]];
    let grid: Vec<f64> = (0..4).map(|i| 20.0 * i as f64 / 3.0).collect();
    let heights: Vec<Vec<f64>> = HEIGHTS.iter().map(|r| r.to_vec()).collect();
    graph_block(&grid, &grid, &heights, &bezier_knots(3), &bezier_knots(3))
}
