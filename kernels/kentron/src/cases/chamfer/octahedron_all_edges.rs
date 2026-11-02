//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with the same distance √6 on
//! both faces (measured across each face, square to the edge), the four chamfers at each corner
//! closed by a flat square: the octahedron cut by the planes |x| + |y| ≤ 8 and the like and by
//! the corner planes |x| ≤ 6 and the like. All twelve in one chamfer, as OCCT's MakeChamfer does.
use crate::bench::edges;
use crate::shared::chamfer::octahedron;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = octahedron(m, 10.0)?;
    let all = edges(m, solid);
    Ok(blend::chamfer(m, solid, &all, 6f64.sqrt())?)
}
