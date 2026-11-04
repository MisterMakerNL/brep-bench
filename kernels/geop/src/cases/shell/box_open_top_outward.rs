//! The box from the origin to (10, 8, 6), shelled outward to a wall 1 thick, open at its top,
//! the wall every point outside the box within 1 of its faces other than the top: the box grown
//! by 1 (its edges quarter cylinders, its corners eighths of a ball) cut off at z = 6, less the
//! box.
//!
//! geop's shell grows walls outward with their faces extended until they meet, sharp, and cannot
//! open such walls at a face. So the box is shelled outward closed all round, the wall's twelve
//! outer edges are rounded to radius 1 (each round about the box's own edge, its corners where
//! three meet eighths of a ball about the box's corners: the box grown by 1), and everything
//! above the plane of the top, z = 6, is cut away: a rectangle drawn on that plane (a datum
//! offset from the xy plane) past the wall, extruded up through it.
use crate::shared::hollow::{block, level, shell};
use crate::shared::{Drawing, extrude, made, sketch};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "box", [0.0, 0.0], [10.0, 8.0], 6.0)?;
    shell(b, "walls", &[], 1.0, true)?;
    let mut outer = Vec::new();
    for z in [-1.0, 7.0] {
        outer.extend([[5.0, -1.0, z], [11.0, 4.0, z], [5.0, 9.0, z], [-1.0, 4.0, z]]);
    }
    outer.extend([[-1.0, -1.0, 3.0], [11.0, -1.0, 3.0], [11.0, 9.0, 3.0], [-1.0, 9.0, 3.0]]);
    let edges = b.edges(&outer)?;
    b.step("rounds", FilletArgs::constant(edges, 1.0))?;
    let top = level(b, "top", 6.0)?;
    let mut lid = Drawing::new();
    lid.rectangle([-5.0, -5.0], [20.0, 20.0]);
    sketch(b, "lid", top, lid)?;
    extrude(b, "open", "lid", 5.0, Combine::cut(made("fillet", "rounds")))
}
