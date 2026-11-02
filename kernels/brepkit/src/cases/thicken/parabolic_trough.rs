//! The parabolic cylinder z = x²/8, −4 ≤ x ≤ 4, 0 ≤ y ≤ 10, as a B-spline sheet (degree 2 in u
//! through (−4, y, 2), (0, y, −2), (4, y, 2), degree 1 in v from y = 0 to y = 10, no interior
//! knots, all weights 1), bounded by its two parabolic arcs and two straight lines, thickened by 1
//! to its concave side. Its normal Su × Sv points up, toward the focus, so it is thickened along it.
use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_math::nurbs::surface::NurbsSurface;
use brepkit_operations::thicken::thicken;
use brepkit_topology::builder::make_line_edge;
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::{Face, FaceSurface};
use brepkit_topology::solid::SolidId;
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let row = |x: f64, z: f64| vec![p(x, 0.0, z), p(x, 10.0, z)];
    let surface = NurbsSurface::new(
        2,
        1,
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![0.0, 0.0, 1.0, 1.0],
        vec![row(-4.0, 2.0), row(0.0, -2.0), row(4.0, 2.0)],
        vec![vec![1.0; 2]; 3],
    )?;
    // the boundary: the parabola at y = 0 along +x, the line at x = 4, the parabola at y = 10
    // back along −x, and the line at x = −4
    let mut parabola = |y: f64| -> Result<_, Refusal> {
        let curve = NurbsCurve::new(
            2,
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            vec![p(-4.0, y, 2.0), p(0.0, y, -2.0), p(4.0, y, 2.0)],
            vec![1.0; 3],
        )?;
        let a = topo.add_vertex(Vertex::new(p(-4.0, y, 2.0), 1e-7));
        let b = topo.add_vertex(Vertex::new(p(4.0, y, 2.0), 1e-7));
        Ok(topo.add_edge(Edge::new(a, b, EdgeCurve::NurbsCurve(curve))))
    };
    let near = parabola(0.0)?;
    let far = parabola(10.0)?;
    let right = make_line_edge(topo, p(4.0, 0.0, 2.0), p(4.0, 10.0, 2.0), 1e-7)?;
    let left = make_line_edge(topo, p(-4.0, 10.0, 2.0), p(-4.0, 0.0, 2.0), 1e-7)?;
    let boundary = Wire::new(
        vec![
            OrientedEdge::new(near, true),
            OrientedEdge::new(right, true),
            OrientedEdge::new(far, false),
            OrientedEdge::new(left, true),
        ],
        true,
    )?;
    let boundary = topo.add_wire(boundary);
    let sheet = topo.add_face(Face::new(boundary, vec![], FaceSurface::Nurbs(surface)));
    Ok(thicken(topo, sheet, 1.0)?)
}
