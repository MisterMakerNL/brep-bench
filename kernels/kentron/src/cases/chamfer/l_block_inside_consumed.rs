//! An L-block: the polygon through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10) and (0, 10) in xz,
//! extruded along +y from y = 0 to y = 10. Its inside edge, the line x = 3, z = 3, is chamfered
//! with the same distance 7 on both faces, adding material: the chamfer is the plane through
//! (10, y, 3) and (3, y, 10), consuming both faces, and the section becomes the pentagon through
//! (0, 0), (10, 0), (10, 3), (3, 10) and (0, 10).
use crate::bench::{edge_through, p, v};
use crate::shared::chamfer::prism;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let section = [(0.0, 0.0), (10.0, 0.0), (10.0, 3.0), (3.0, 3.0), (3.0, 10.0), (0.0, 10.0)];
    let points: Vec<_> = section.iter().map(|&(x, z)| p(x, 0.0, z)).collect();
    let block = prism(m, &points, v(0.0, 10.0, 0.0))?;
    let inside = edge_through(m, block, p(3.0, 5.0, 3.0))?;
    Ok(blend::chamfer(m, block, &[inside], 7.0)?)
}
