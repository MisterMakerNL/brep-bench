//! A ball of radius 5 bounded by one NURBS face: brepkit's rational NURBS surface on the case's
//! 9 × 5 net, the tensor product of the rational quadratic unit circle (round the axis, u) and the
//! rational quadratic semicircle from the south pole to the north (v), its control points
//! (ρⱼcᵢ, zⱼ) weighted wᵢvⱼ. The face is bounded by its seam, the meridian at u = 0 (the rational
//! quadratic on the net's first row, from the south pole to the north), run up and back down: its
//! edges v = 0 and v = 1 are the poles, single vertices. The one face closes a shell.
use brepkit_math::vec::Point3;
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::{Face, FaceSurface};
use brepkit_topology::shell::Shell;
use brepkit_topology::solid::{Solid, SolidId};
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::{nurbs, surface, TOL};
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    // round the axis: the unit circle cᵢ with weights wᵢ
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
    // along the meridian: (ρⱼ, zⱼ) with weights vⱼ
    let meridian = [(0.0, -5.0), (5.0, -5.0), (5.0, 0.0), (5.0, 5.0), (0.0, 5.0)];
    let vw = [1.0, h, 1.0, h, 1.0];
    let knots_v = [0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0];

    let net = c
        .iter()
        .map(|&(cx, cy)| {
            meridian
                .iter()
                .map(|&(rho, z)| p(rho * cx, rho * cy, z))
                .collect()
        })
        .collect();
    let weights = w
        .iter()
        .map(|&wi| vw.iter().map(|&vj| wi * vj).collect())
        .collect();
    let ball = surface(2, 2, &knots_u, &knots_v, net, weights)?;

    // the seam: the net's first row (c₀ = (1, 0)), from the south pole to the north
    let seam: Vec<Point3> = meridian.iter().map(|&(rho, z)| p(rho, 0.0, z)).collect();
    let south = topo.add_vertex(Vertex::new(seam[0], TOL));
    let north = topo.add_vertex(Vertex::new(seam[4], TOL));
    let seam = nurbs(2, &knots_v, &seam, &vw)?;
    let seam = topo.add_edge(Edge::new(south, north, EdgeCurve::NurbsCurve(seam)));
    let wire = topo.add_wire(Wire::new(
        vec![
            OrientedEdge::new(seam, true),
            OrientedEdge::new(seam, false),
        ],
        true,
    )?);
    let face = topo.add_face(Face::new(wire, vec![], FaceSurface::Nurbs(ball)));
    let shell = topo.add_shell(Shell::new(vec![face])?);
    Ok(topo.add_solid(Solid::new(shell, vec![])))
}
