//! The solid between two discs of radius 5 at z = ±5, its side ruled between their rims a quarter
//! turn apart: a hyperboloid of one sheet built as a ruled surface.
use cad_kernel::build::{disc, face, grid_sheet};
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const R: f64 = 5.;
const HALF: f64 = 5.;
const H: f64 = FRAC_1_SQRT_2;
const CIRCLE: [[f64; 2]; 9] = [
    [1., 0.], [1., 1.], [0., 1.], [-1., 1.], [-1., 0.], [-1., -1.], [0., -1.], [1., -1.], [1., 0.],
];
const CIRCLE_W: [f64; 9] = [1., H, 1., H, 1., H, 1., H, 1.];

pub fn build() -> Result<Brep, String> {
    // the ruled surface between two rational circles: a net linear in v whose rows are the rims.
    // Each rim's poles are weighted alike, so the line at u joins the lower rim's point at u to
    // the upper's: the upper rim's poles turned a quarter turn, (x, y) to (−y, x), carry the point
    // at θ below to θ + 90° above
    let poles = CIRCLE.iter()
        .map(|c| vec![[R * c[0], R * c[1], -HALF], [-R * c[1], R * c[0], HALF]])
        .collect();
    let weights = CIRCLE_W.iter().map(|&w| vec![w, w]).collect();
    let net = Net {
        du: 2,
        dv: 1,
        uknots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        vknots: vec![0., 0., 1., 1.],
        poles,
        weights: Some(weights),
    };
    // the side as one sheet closed round u, in two faces, its ends the two discs, sewn shut
    let names = vec!["east".to_string(), "west".to_string()];
    let side = grid_sheet(&net, &[0., 0.5, 1.], &[0., 1.], &names, [true, false])?;
    let floor = face(&disc([0., 0., -HALF], [0., 0., -1.], R))?;
    let roof = face(&disc([0., 0., HALF], [0., 0., 1.], R))?;
    Ok(sewn(&[side, floor, roof], tolerance(2. * R))?)
}
