//! A box from the origin to (a, b, c), the four edges round its top chamfered d1 across the top and
//! d2 down the side, one after another; turned by ψ about the axis (θ, φ).
use crate::sketch::{polygon, turned_by};
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

pub fn build(p: &Params) -> Result<Brep, String> {
    let (a, b, c) = (p.num("a")?, p.num("b")?, p.num("c")?);
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // chamfer reads faces of one name as pieces of one face
    let mut floor = polygon(&[[0., 0., 0.], [a, 0., 0.], [a, b, 0.], [0., b, 0.]]);
    floor.names = vec![["front", "right", "back", "left"].map(String::from).to_vec()];
    let mut part = prism(&floor, 0., c)?;
    // the middle of the top face says which face of each edge takes the first leg (d1, measured
    // from the side's plane); the second, d2 from the top's plane, is on the side
    let top = [a / 2., b / 2., c];
    for at in [[a / 2., 0., c], [a, b / 2., c], [a / 2., b, c], [0., b / 2., c]] {
        let legs = [p.num("d1")?, p.num("d2")?];
        let piece = flat_along(&part, at, legs, Some(top), tolerance(part.size()))?;
        part = combined(&part, &piece, Op::Cut, 0.)?;
    }
    Ok(turned_by(&part, p.num("theta")?, p.num("phi")?, p.num("psi")?))
}
