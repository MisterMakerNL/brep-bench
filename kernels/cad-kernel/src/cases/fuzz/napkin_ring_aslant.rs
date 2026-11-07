//! A ball of radius R about (cx, cy, cz), bored right through along the line through its centre
//! with direction (θ, φ) by a hole of radius r.
use crate::sketch::direction;
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, sphere};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{scale, sub};

pub fn build(p: &Params) -> Result<Brep, String> {
    let (big, small) = (p.num("R")?, p.num("r")?);
    let centre = [p.num("cx")?, p.num("cy")?, p.num("cz")?];
    let along = direction(p.num("theta")?, p.num("phi")?);
    // the ball, its poles on the vertical through its centre
    let ball = sphere(centre, [0., 0., 1.], big)?;
    // a drill reaching past the ball both ways along the hole's axis
    let drill = prism(&disc(sub(centre, scale(along, 2. * big)), along, small), 0., 4. * big)?;
    Ok(combined(&ball, &drill, Op::Cut, 0.)?)
}
