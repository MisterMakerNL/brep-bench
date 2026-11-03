//! The box from the origin to (40, 30, 10), the edge where its top, z = 10, meets its back,
//! y = 30, rounded to radius 3; its top then moved up by 6, to z = 16, carrying the round with it.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, Rigid};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // round reads faces of one name as pieces of one face
    let mut floor = polygon(&[[0., 0., 0.], [40., 0., 0.], [40., 30., 0.], [0., 30., 0.]]);
    floor.names = vec![["front", "right", "back", "left"].map(String::from).to_vec()];
    let block = prism(&floor, 0., 10.)?;
    // the edge is picked by its middle; a convex edge's round takes its piece away
    let round = roll(&block, [20., 30., 10.], 3., tolerance(block.size()))?;
    let rounded = combined(&block, &round.piece, Op::Cut, 0.)?;
    // cad-kernel moves no face of a solid. Every vertical line through the block meets it in one
    // run from the floor up to the top or the round, at least 7 tall, so the run and its copy
    // raised by 6 overlap, and together run from the floor up to 6 above the top or the round:
    // the block with its top and round carried up is the block joined to itself raised by 6.
    let up = Rigid { t: [0., 0., 6.], ..Rigid::identity() };
    Ok(combined(&rounded, &rounded.moved(&up), Op::Union, 0.)?)
}
