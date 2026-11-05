//! A ball of radius 10 about the origin with 62 dimples, each cut by its own ball of radius 1.2
//! centered 10.6 from the origin toward one of the 12 vertices, 30 edge midpoints and 20 face
//! centers of the icosahedron with vertices (0, ±1, ±φ), (±1, ±φ, 0), (±φ, 0, ±1). Every ball has
//! its poles on the vertical through its center.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid, Vec3};

/// The 62 unit directions: the icosahedron's vertices, the middles of its edges (pairs of vertices
/// at its edge length, 2) and the centers of its faces (triples of mutually adjacent vertices).
fn directions() -> Vec<Vec3> {
    let phi = (1.0 + 5f64.sqrt()) / 2.0;
    let mut vertices = Vec::new();
    for a in [1.0, -1.0] {
        for b in [phi, -phi] {
            vertices.extend([p(0.0, a, b), p(a, b, 0.0), p(b, 0.0, a)]);
        }
    }
    let adjacent = |i: usize, j: usize| (vertices[i].sub(vertices[j]).length() - 2.0).abs() < 1e-9;
    let mut out: Vec<Vec3> = vertices.clone();
    let n = vertices.len();
    for i in 0..n {
        for j in i + 1..n {
            if adjacent(i, j) {
                out.push(vertices[i].add(vertices[j]));
                for k in j + 1..n {
                    if adjacent(i, k) && adjacent(j, k) {
                        out.push(vertices[i].add(vertices[j]).add(vertices[k]));
                    }
                }
            }
        }
    }
    out.into_iter().map(|d| d.normalized_or(Vec3::Z)).collect()
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let mut ball = make_sphere_brep(p(0.0, 0.0, 0.0), 10.0, up)?;
    let dirs = directions();
    if dirs.len() != 62 {
        return Err(Refusal::from(format!("{} directions, not 62", dirs.len())));
    }
    for d in dirs {
        let dimple = make_sphere_brep(d.scale(10.6), 1.2, up)?;
        ball = cut(&ball, &dimple)?;
    }
    Ok(ball)
}
