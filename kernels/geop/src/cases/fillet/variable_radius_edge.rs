//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded with a radius growing linearly along it, 1 at z = 0 to 2 at z = 10: the
//! cube extruded, and a variable fillet of that edge, its radius set at the edge's two ends, each
//! vertex picked by its point (geop's radius runs linearly between them).
use crate::shared::blends::{element, vertex};
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::{FilletArgs, VertexRadius};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let edge = b.edge([10.0, 10.0, 5.0])?;
    let bottom = vertex(b, [10.0, 10.0, 0.0])?;
    let top = vertex(b, [10.0, 10.0, 10.0])?;
    let radii = vec![
        VertexRadius { vertex: element(bottom), radius: 1.0 },
        VertexRadius { vertex: element(top), radius: 2.0 },
    ];
    b.step("round", FilletArgs { vertex_radii: radii, ..FilletArgs::constant(vec![edge], 1.0) })
}
