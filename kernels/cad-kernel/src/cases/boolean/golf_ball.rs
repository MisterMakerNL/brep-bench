//! A ball of radius 10 at the origin with 62 dimples, each cut by a ball of radius 1.2 centred
//! 10.6 out towards a vertex, an edge's midpoint or a face's centre of the icosahedron with
//! vertices (0, ±1, ±φ), (±1, ±φ, 0), (±φ, 0, ±1); every ball's poles on the vertical through
//! its centre, as the case asks.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, distance, scale, unit};

const RADIUS: f64 = 10.;
const DIMPLE_RADIUS: f64 = 1.2;
const DIMPLE_CENTRE: f64 = 10.6;
const UP: V = [0., 0., 1.];

/// The 62 directions: the icosahedron's 12 vertices, its 30 edges' midpoints and its 20 faces'
/// centres, each as a unit vector from its centre.
fn directions() -> Vec<V> {
    let phi = (1. + 5f64.sqrt()) / 2.;
    let mut vertices = Vec::new();
    for a in [-1., 1.] {
        for b in [-phi, phi] {
            vertices.extend([[0., a, b], [a, b, 0.], [b, 0., a]]);
        }
    }
    // neighbours are an edge, 2, apart
    let edge = |i: usize, j: usize| (distance(vertices[i], vertices[j]) - 2.).abs() < 1e-9;
    let mut out: Vec<V> = vertices.iter().map(|&v| unit(v)).collect();
    for i in 0..12 {
        for j in i + 1..12 {
            if edge(i, j) {
                out.push(unit(add(vertices[i], vertices[j])));
            }
        }
    }
    for i in 0..12 {
        for j in i + 1..12 {
            for k in j + 1..12 {
                if edge(i, j) && edge(j, k) && edge(i, k) {
                    out.push(unit(add(add(vertices[i], vertices[j]), vertices[k])));
                }
            }
        }
    }
    out
}

pub fn build() -> Result<Brep, String> {
    let mut ball = sphere([0., 0., 0.], UP, RADIUS)?;
    let dimples = directions();
    assert_eq!(dimples.len(), 62);
    for d in dimples {
        // each dimple its own cut
        let cutter = sphere(scale(d, DIMPLE_CENTRE), UP, DIMPLE_RADIUS)?;
        ball = combined(&ball, &cutter, Op::Cut, 0.)?;
    }
    Ok(ball)
}
