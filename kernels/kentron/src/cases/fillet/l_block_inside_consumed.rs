//! An L-block: the polygon through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10) and (0, 10) in xz,
//! extruded along +y from y = 0 to y = 10. Its inside edge, the line x = 3, z = 3, is rounded to
//! radius 7, adding material: the round consumes both faces at the edge, and the section becomes
//! the square from (0, 0) to (10, 10) less the quarter disc of radius 7 about (10, 10).
use crate::bench::{edge_through, p, v};
use crate::shared::fillet::prism;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let section = [(0.0, 0.0), (10.0, 0.0), (10.0, 3.0), (3.0, 3.0), (3.0, 10.0), (0.0, 10.0)];
    let points: Vec<_> = section.iter().map(|&(x, z)| p(x, 0.0, z)).collect();
    let block = prism(m, &points, v(0.0, 10.0, 0.0))?;
    let inside = edge_through(m, block, p(3.0, 5.0, 3.0))?;
    Ok(blend::fillet(m, block, &[inside], 7.0)?)
}
