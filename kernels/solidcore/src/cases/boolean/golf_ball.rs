//! A ball of radius 10 at the origin with 62 dimples, each cut by a ball of radius 1.2 centred
//! 10.6 out towards a vertex, an edge's midpoint or a face's centre of the icosahedron with
//! vertices (0, ±1, ±φ), (±1, ±φ, 0), (±φ, 0, ±1); each cut its own. solidcore's balls have
//! their poles on the z axis through their centres, on the vertical, as the case asks.
//!
//! The cuts are the kernel's own booleans, without the checks ops makes of every operand and
//! result, which on this body take nearly all the time 62 cuts take
//! ([`cut_all_in_kernel`]): through ops the case runs past two minutes.
use crate::shared::boolean::cut_all_in_kernel;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

/// The 62 directions, as unit vectors: the icosahedron's 12 vertices, its 30 edges' midpoints
/// and its 20 faces' centres.
fn directions() -> Vec<[f64; 3]> {
    let phi = (1.0 + 5f64.sqrt()) / 2.0;
    let mut vertices = Vec::new();
    for a in [-1.0, 1.0] {
        for b in [-phi, phi] {
            vertices.extend([[0.0, a, b], [a, b, 0.0], [b, 0.0, a]]);
        }
    }
    let sum = |ps: &[[f64; 3]]| {
        let s = ps.iter().fold([0.0; 3], |s, p| [s[0] + p[0], s[1] + p[1], s[2] + p[2]]);
        let l = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
        s.map(|x| x / l)
    };
    // neighbours are an edge, 2, apart
    let edge = |i: usize, j: usize| {
        let (a, b): ([f64; 3], [f64; 3]) = (vertices[i], vertices[j]);
        let d = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
        (d - 2.0).abs() < 1e-9
    };
    let mut out: Vec<_> = vertices.iter().map(|&v| sum(&[v])).collect();
    for i in 0..12 {
        for j in i + 1..12 {
            if edge(i, j) {
                out.push(sum(&[vertices[i], vertices[j]]));
            }
        }
    }
    for i in 0..12 {
        for j in i + 1..12 {
            for k in j + 1..12 {
                if edge(i, j) && edge(j, k) && edge(i, k) {
                    out.push(sum(&[vertices[i], vertices[j], vertices[k]]));
                }
            }
        }
    }
    out
}

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 10.0)?;
    let dimples = directions()
        .into_iter()
        .map(|d| make_sphere(d.map(|x| 10.6 * x), 1.2))
        .collect::<solidcore_ops::Result<Vec<_>>>()?;
    cut_all_in_kernel(ball, dimples)
}
