//! A solid cone whose side is one NURBS face with a pole at its apex. The side is brepkit's
//! rational NURBS surface on the case's 9 × 2 net (the base circle's rational quadratic as its
//! first row, nine copies of the apex as its second, the same weights in both). Its face is bounded
//! the way brepkit's own pointed cone is: the base circle (the rational quadratic on the first row,
//! one closed edge), then the seam line from the circle up to the apex and back down it; the base
//! is the flat disc on the same circle edge, run the other way. The two faces close a shell.
use brepkit_math::vec::Point3;
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::{Face, FaceSurface};
use brepkit_topology::shell::Shell;
use brepkit_topology::solid::{Solid, SolidId};
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::{nurbs, surface, TOL};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let c = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let w = [1.0, h, 1.0, h, 1.0, h, 1.0, h, 1.0];
    let knots_u = [
        0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
    ];
    let apex = p(0.0, 0.0, 10.0);
    let base: Vec<Point3> = c.iter().map(|&(x, y)| p(5.0 * x, 5.0 * y, 0.0)).collect();
    // row i of the net (along u) is the column of its two points along v: base, then apex
    let net = base.iter().map(|&b| vec![b, apex]).collect();
    let weights = w.iter().map(|&wi| vec![wi, wi]).collect();
    let side = surface(2, 1, &knots_u, &[0.0, 0.0, 1.0, 1.0], net, weights)?;

    let v_base = topo.add_vertex(Vertex::new(base[0], TOL));
    let v_apex = topo.add_vertex(Vertex::new(apex, TOL));
    let rim = nurbs(2, &knots_u, &base, &w)?;
    let rim = topo.add_edge(Edge::new(v_base, v_base, EdgeCurve::NurbsCurve(rim)));
    let seam = topo.add_edge(Edge::new(v_base, v_apex, EdgeCurve::Line));
    let side_wire = Wire::new(
        vec![
            OrientedEdge::new(rim, true),
            OrientedEdge::new(seam, true),
            OrientedEdge::new(seam, false),
        ],
        true,
    )?;
    let side_wire = topo.add_wire(side_wire);
    let side = topo.add_face(Face::new(side_wire, vec![], FaceSurface::Nurbs(side)));
    let floor_wire = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, false)], true)?);
    let floor = topo.add_face(Face::new(
        floor_wire,
        vec![],
        FaceSurface::Plane {
            normal: v(0.0, 0.0, -1.0),
            d: 0.0,
        },
    ));
    let shell = topo.add_shell(Shell::new(vec![side, floor])?);
    Ok(topo.add_solid(Solid::new(shell, vec![])))
}
