//! A ball of radius 10 centred on the origin with 62 dimples, each cut by a ball of radius 1.2
//! whose centre is 10.6 from the origin towards a vertex, the midpoint of an edge or the centre
//! of a face of the regular icosahedron with vertices (0, ±1, ±φ), (±1, ±φ, 0), (±φ, 0, ±1).
//! geop has no ball of its own: every ball is a half disc turned about the vertical through its
//! centre (so its poles are on that vertical), drawn in the plane through its centre parallel to
//! the xz plane. The 62 small balls are cut from the big one one after another, each its own cut.
use crate::shared::booleans::{ball, boolean};
use crate::{Bench, Refusal};
use geop_ops_model::BooleanOp;

/// The 62 directions: the icosahedron's 12 vertices, its 30 edges' midpoints (between vertices
/// 2 apart) and its 20 faces' centres (of three vertices each 2 from the others).
fn directions() -> Vec<[f64; 3]> {
    let phi = (1.0 + 5f64.sqrt()) / 2.0;
    let mut vertices = Vec::new();
    for s in [1.0, -1.0] {
        for t in [1.0, -1.0] {
            vertices.extend([[0.0, s, t * phi], [s, t * phi, 0.0], [s * phi, 0.0, t]]);
        }
    }
    let near = |a: [f64; 3], b: [f64; 3]| {
        let d: f64 = (0..3).map(|k| (a[k] - b[k]).powi(2)).sum();
        (d - 4.0).abs() < 1e-9
    };
    let mean = |ps: &[[f64; 3]]| {
        let n = ps.len() as f64;
        [0, 1, 2].map(|k| ps.iter().map(|p| p[k]).sum::<f64>() / n)
    };
    let mut out = vertices.clone();
    let count = vertices.len();
    for i in 0..count {
        for j in i + 1..count {
            if near(vertices[i], vertices[j]) {
                out.push(mean(&[vertices[i], vertices[j]]));
            }
        }
    }
    for i in 0..count {
        for j in i + 1..count {
            for k in j + 1..count {
                let (a, b, c) = (vertices[i], vertices[j], vertices[k]);
                if near(a, b) && near(b, c) && near(a, c) {
                    out.push(mean(&[a, b, c]));
                }
            }
        }
    }
    out
}

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut body = ball(b, "ball", [0.0, 0.0, 0.0], 10.0)?;
    for (i, d) in directions().into_iter().enumerate() {
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let center = d.map(|c| 10.6 * c / length);
        let dimple = ball(b, &format!("dimple{i}"), center, 1.2)?;
        let cut = format!("cut{i}");
        boolean(b, &cut, &body, &dimple, BooleanOp::Difference)?;
        body = format!("boolean({cut})");
    }
    Ok(())
}
