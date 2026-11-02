//! The solid between two discs of radius 5 at z = ±5, its side ruled between their rims a quarter
//! turn apart: a hyperboloid of one sheet built as a ruled surface.
use cad_kernel::build::{disc, face, grid_sheet};
use cad_kernel::geom::{tolerance, Frame};
use cad_kernel::nurbs::{arc, Net};
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::f64::consts::{PI, TAU};

const R: f64 = 5.;
const HALF: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // the rims as rational circles, the upper one starting a quarter turn round (its frame's x
    // along y), so the point at θ below and the point at θ + 90° above share a parameter
    let up = [0., 0., 1.];
    let lower = arc(&Frame::new([0., 0., -HALF], up, [1., 0., 0.]), R, [0., TAU]);
    let upper = arc(&Frame::new([0., 0., HALF], up, [0., 1., 0.]), R, [0., TAU]);
    // the ruled surface between them: a net linear in v whose first and last columns are the
    // rims. Their weights are alike, so the line at each parameter joins the two points on it
    let net = Net {
        du: 2,
        dv: 1,
        uknots: lower.knots.clone(),
        vknots: vec![0., 0., 1., 1.],
        poles: lower.poles.iter().zip(&upper.poles).map(|(&a, &b)| vec![a, b]).collect(),
        weights: lower.weights.as_ref().map(|w| w.iter().map(|&w| vec![w, w]).collect()),
    };
    // the side as one sheet closed round u, in two faces, its ends the two discs, sewn shut
    let names = vec!["east".to_string(), "west".to_string()];
    let side = grid_sheet(&net, &[0., PI, TAU], &[0., 1.], &names, [true, false])?;
    let floor = face(&disc([0., 0., -HALF], [0., 0., -1.], R))?;
    let roof = face(&disc([0., 0., HALF], up, R))?;
    let tol = tolerance(side.size());
    Ok(sewn(&[side, floor, roof], tol)?)
}
