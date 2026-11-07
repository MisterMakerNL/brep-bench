//! A pipe of radius r along n turns of the helix of radius R about z that rises p a turn, from
//! (R, 0, 0), right- or left-handed; flat ends.
use crate::Params;
use cad_kernel::build::{canal, cut_before};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{scale, unit};
use std::f64::consts::TAU;

pub fn build(p: &Params) -> Result<Brep, String> {
    let (big, r, pitch, turns) = (p.num("R")?, p.num("r")?, p.num("p")?, p.num("n")?);
    // a left-handed helix turns the other way about z as it rises
    let hand = match p.text("hand")? {
        "right" => 1.,
        "left" => -1.,
        other => return Err(format!("no hand `{other}`")),
    };
    let start = [big, 0., 0.];
    let angle = hand * turns * TAU;
    let end = [big * angle.cos(), big * angle.sin(), turns * pitch];
    // the ball swept along the helix: the disc square to the path screwed along it, with the half
    // ball the ball leaves beyond each end disc
    let rolled = canal(start, r, [0., 0., 0.], [0., 0., 1.], angle, turns * pitch, "pipe")?;
    // the path's tangent at each end, square to which its end disc lies
    let tangent = |a: f64| unit([-hand * TAU * big * a.sin(), hand * TAU * big * a.cos(), pitch]);
    // the half balls cut off at those discs, the cutters reaching no further than a ball, short of
    // the next turn
    let reach = 1.05 * r;
    let rolled = cut_before(&rolled, start, tangent(0.), reach, 0.)?;
    cut_before(&rolled, end, scale(tangent(angle), -1.), reach, 0.)
}
