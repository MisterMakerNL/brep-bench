//! A cone on the disc of radius 5 in the plane z = 0, its apex at (0, 0, 10), its side a 9 × 2
//! rational net whose second row collapses to the apex.
use cad_kernel::build::{disc, face, grid_sheet};
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const R: f64 = 5.;
const APEX: f64 = 10.;
const H: f64 = FRAC_1_SQRT_2;
const CIRCLE: [[f64; 2]; 9] = [
    [1., 0.], [1., 1.], [0., 1.], [-1., 1.], [-1., 0.], [-1., -1.], [0., -1.], [1., -1.], [1., 0.],
];
const CIRCLE_W: [f64; 9] = [1., H, 1., H, 1., H, 1., H, 1.];

pub fn build() -> Result<Brep, String> {
    // the control point (i, 0) is 5cᵢ on the floor, (i, 1) the apex, both weighted wᵢ
    let poles = CIRCLE.iter().map(|c| vec![[R * c[0], R * c[1], 0.], [0., 0., APEX]]).collect();
    let weights = CIRCLE_W.iter().map(|&w| vec![w, w]).collect();
    let net = Net {
        du: 2,
        dv: 1,
        uknots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        vknots: vec![0., 0., 1., 1.],
        poles,
        weights: Some(weights),
    };
    // the side as one sheet closed round u, in two faces; its edge at v = 1 is the apex, a pole
    let names = vec!["east".to_string(), "west".to_string()];
    let side = grid_sheet(&net, &[0., 0.5, 1.], &[0., 1.], &names, [true, false])?;
    let base = face(&disc([0., 0., 0.], [0., 0., -1.], R))?;
    let tol = tolerance(side.size());
    Ok(sewn(&[side, base], tol)?)
}
