//! What the geometry implementations share: a section's face from its edges, the solid it turns
//! about the z axis, and the solid bounded by faces sewn into one closed shell.
use crate::Refusal;
use kentron::geometry::curve::Curve;
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::make;
use kentron::math::{Axis, Point, Vector};
use kentron::properties;
use kentron::sew::{self, SewOptions};
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};
use std::f64::consts::TAU;

/// An edge along `curve` from parameter `first` to `last`.
pub fn arc(m: &mut Model, curve: Curve, first: f64, last: f64) -> kentron::Result<Shape> {
    make::edge_from_curve_range(m, curve, first, last)
}

/// A straight edge from `a` to `b`.
pub fn segment(m: &mut Model, a: Point, b: Point) -> kentron::Result<Shape> {
    make::edge_between(m, a, b)
}

/// The flat face bounded by `edges`, which run one after another round its boundary.
pub fn flat(m: &mut Model, edges: &[Shape]) -> kentron::Result<Shape> {
    let wire = make::wire(m, edges)?;
    make::planar_face(m, wire)
}

/// The solid `section` (a face in a plane through the z axis) fills as it turns a whole turn
/// about the z axis.
pub fn turned(m: &mut Model, section: Shape) -> kentron::Result<Shape> {
    Ok(sweep::revolve(m, section, Axis::Z, TAU)?.shape)
}

/// The saddle z = 10 + xy/8 as the bilinear B-spline patch, degree 1 each way, on the corners
/// of the square −10 ≤ x, y ≤ 10 at its heights there, its face swept straight down past the
/// floor: a column to trim with the drum.
pub fn saddle(m: &mut Model) -> kentron::Result<Shape> {
    let poles = vec![
        vec![Point::new(-10.0, -10.0, 22.5), Point::new(-10.0, 10.0, -2.5)],
        vec![Point::new(10.0, -10.0, -2.5), Point::new(10.0, 10.0, 22.5)],
    ];
    let linear = || KnotVector::new(vec![0.0, 1.0], vec![2, 2], 1);
    let patch = BSplineSurface::try_new(poles, linear(), linear())?;
    let top = make::face_on_surface(m, Surface::BSpline(patch))?;
    Ok(sweep::extrude(m, top, Vector::new(0.0, 0.0, -40.0))?.shape)
}

/// The unit circle about the z axis as the rational quadratic the NURBS cases give: nine poles
/// (x, y) and their weights on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1), the last pole
/// the first again.
///
/// kentron makes a face's seam only where its surface is periodic (`make::face_on_surface`), and
/// its sewing will not join a face's two edges that run between the same two vertices, as the
/// edges u = 0 and u = 1 of a surface with a pole do: on the clamped net the face stays open
/// along its seam. So the circle is taken as the same curve in its periodic form, which closes
/// on itself: the first eight poles and weights, on the knots 0, ¼, ½, ¾, 1, each twice, a
/// period of 1. Returns those poles, weights, knots and multiplicities.
pub fn circle_nurbs_periodic() -> (Vec<(f64, f64)>, Vec<f64>, Vec<f64>, Vec<usize>) {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let poles = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];
    let weights = [1.0, h, 1.0, h, 1.0, h, 1.0, h, 1.0];
    // the clamped circle's ninth pole is its first: the periodic one wraps round to it
    (poles[..8].to_vec(), weights[..8].to_vec(), vec![0.0, 0.25, 0.5, 0.75, 1.0], vec![2; 5])
}

/// The solid bounded by `faces`, sewn edge to edge into one closed shell.
pub fn solid_of(m: &mut Model, faces: &[Shape]) -> Result<Shape, Refusal> {
    let sewn = sew::sew(m, faces, &SewOptions { tolerance: 1e-6, ..SewOptions::default() })?;
    let shell = if sewn.kind() == NodeKind::Face {
        // one face sewn to itself (along a seam) is returned as a face: its shell is that face
        let shell = m.new_empty(NodeKind::Shell);
        m.add(shell, sewn)?;
        shell
    } else {
        let shells = m.unique_subshapes(sewn, NodeKind::Shell);
        let [shell] = shells[..] else {
            return Err(format!("sewing made {} shells of the faces, not one", shells.len()).into());
        };
        shell
    };
    let solid = m.new_empty(NodeKind::Solid);
    m.add(solid, shell)?;
    if properties::volume(m, solid).mass >= 0.0 {
        return Ok(solid);
    }
    // the shell faces inward: the solid is bounded by it reversed
    let solid = m.new_empty(NodeKind::Solid);
    m.add(solid, shell.reversed())?;
    Ok(solid)
}
