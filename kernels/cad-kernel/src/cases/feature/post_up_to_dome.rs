//! A closed hollow dome with a post inside it. The dome: the disc of radius 20 about z from
//! z = −2 to 0, and on it everything above z = 0 between 18 and 20 from the origin, which closes a
//! half-ball cavity of radius 18 on the plate. The post: the disc of radius 3 on the cavity's
//! floor, extruded up until it meets the dome's inside, and joined to the dome.
//!
//! cad-kernel's prism runs between two given heights and has no extrusion up to a face. So the
//! disc is extruded up 19, past the inside sphere (at most 18 over the disc) and short of the
//! outside one (at least √391 ≈ 19.77 over it): the post's top lies in the dome's wall, and the
//! union leaves the post ending on the dome's inside.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const OUTSIDE: f64 = 20.;
const INSIDE: f64 = 18.;
const PLATE: f64 = 2.;
const POST_RADIUS: f64 = 3.;
const POST_REACH: f64 = 19.;

pub fn build() -> Result<Brep, String> {
    // the dome's section in the half-plane y = 0, x ≥ 0: the plate's rectangle and the quarter
    // annulus on it, the arcs turning from +x up to +z about −y (the profile's normal)
    let quarter = Frame::new([0., 0., 0.], [0., -1., 0.], [1., 0., 0.]);
    let arc = |r: f64| ProfileEdge::Arc { frame: quarter, r, span: Some([0., FRAC_PI_2]) };
    let line = |a: [f64; 2], b: [f64; 2]| ProfileEdge::Line { a: [a[0], 0., a[1]], b: [b[0], 0., b[1]] };
    let section = Profile {
        origin: [0., 0., 0.],
        normal: [0., -1., 0.],
        loops: vec![vec![
            line([0., -PLATE], [OUTSIDE, -PLATE]),
            line([OUTSIDE, -PLATE], [OUTSIDE, 0.]),
            arc(OUTSIDE),
            line([0., OUTSIDE], [0., INSIDE]),
            arc(INSIDE),
            line([INSIDE, 0.], [0., 0.]),
            line([0., 0.], [0., -PLATE]),
        ]],
        names: Vec::new(),
    };
    let dome = revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)?;
    let post = prism(&disc([0., 0., 0.], [0., 0., 1.], POST_RADIUS), 0., POST_REACH)?;
    Ok(combined(&dome, &post, Op::Union, 0.)?)
}
