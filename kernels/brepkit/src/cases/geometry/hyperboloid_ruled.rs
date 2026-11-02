//! A hyperboloid of one sheet as a ruled surface. The rims are the circles of radius 5 about the z
//! axis at z = −5 and z = 5, each as brepkit's exact rational NURBS of a whole circle
//! (`circle_to_nurbs`), the lower one starting at angle 0, (5, 0, −5), and the upper at 90°,
//! (0, 5, 5), both running anticlockwise about +z. The side is the NURBS surface ruled between
//! them, linear from each of the lower rim's control points to the upper rim's matching one (the
//! same weights), so its line at each parameter joins the point at θ below to the one at θ + 90°
//! above. Its face is bounded by the two rims and its seam, the line from (5, 0, −5) to (0, 5, 5),
//! run up and back down; the two discs are flat faces on the rims. The three faces close a shell.
//!
//! brepkit's `loft` of the two discs would give a cylinder: it turns its profiles to start at
//! matching angles, so it cannot rule between the rims as they are given, and brepkit has no
//! other operation that rules a surface between two curves.
use brepkit_geometry::convert::circle_to_nurbs;
use brepkit_math::curves::Circle3D;
use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_math::vec::Vec3;
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::{Face, FaceSurface};
use brepkit_topology::shell::Shell;
use brepkit_topology::solid::{Solid, SolidId};
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::{surface, TOL};
use crate::solids::{p, v};
use crate::Refusal;

/// The circle of radius 5 about the z axis at height `z`, starting towards `start`, as a NURBS.
fn rim(z: f64, start: Vec3) -> Result<NurbsCurve, Refusal> {
    let circle = Circle3D::new_with_ref(p(0.0, 0.0, z), v(0.0, 0.0, 1.0), 5.0, start)?;
    circle_to_nurbs(&circle, 0.0, std::f64::consts::TAU).map_err(|e| e.to_string().into())
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let lower = rim(-5.0, v(1.0, 0.0, 0.0))?;
    let upper = rim(5.0, v(0.0, 1.0, 0.0))?;
    // the ruled surface: degree 2 round (u, the rims' own knots), 1 up (v)
    let net = lower
        .control_points()
        .iter()
        .zip(upper.control_points())
        .map(|(&a, &b)| vec![a, b])
        .collect();
    let weights = lower.weights().iter().map(|&w| vec![w, w]).collect();
    let side = surface(2, 1, lower.knots(), &[0.0, 0.0, 1.0, 1.0], net, weights)?;

    let (a, b) = (p(5.0, 0.0, -5.0), p(0.0, 5.0, 5.0));
    let (va, vb) = (
        topo.add_vertex(Vertex::new(a, TOL)),
        topo.add_vertex(Vertex::new(b, TOL)),
    );
    let lower = topo.add_edge(Edge::new(va, va, EdgeCurve::NurbsCurve(lower)));
    let upper = topo.add_edge(Edge::new(vb, vb, EdgeCurve::NurbsCurve(upper)));
    let seam = topo.add_edge(Edge::new(va, vb, EdgeCurve::Line));
    let side_wire = topo.add_wire(Wire::new(
        vec![
            OrientedEdge::new(lower, true),
            OrientedEdge::new(seam, true),
            OrientedEdge::new(upper, false),
            OrientedEdge::new(seam, false),
        ],
        true,
    )?);
    let side = topo.add_face(Face::new(side_wire, vec![], FaceSurface::Nurbs(side)));
    let bottom_wire = topo.add_wire(Wire::new(vec![OrientedEdge::new(lower, false)], true)?);
    let bottom = topo.add_face(Face::new(
        bottom_wire,
        vec![],
        FaceSurface::Plane {
            normal: v(0.0, 0.0, -1.0),
            d: 5.0,
        },
    ));
    let top_wire = topo.add_wire(Wire::new(vec![OrientedEdge::new(upper, true)], true)?);
    let top = topo.add_face(Face::new(
        top_wire,
        vec![],
        FaceSurface::Plane {
            normal: v(0.0, 0.0, 1.0),
            d: 5.0,
        },
    ));
    let shell = topo.add_shell(Shell::new(vec![side, bottom, top])?);
    Ok(topo.add_solid(Solid::new(shell, vec![])))
}
