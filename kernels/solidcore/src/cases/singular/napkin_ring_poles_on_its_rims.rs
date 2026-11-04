//! A ball of radius 5 centred on the origin, bored right through along the z axis by a hole of
//! radius 3, its rims the circles of radius 3 at z = ±4. solidcore's ball has its poles on its z
//! axis, so it is turned about the y axis until they lie along (3, 0, 4), on the rims: at
//! (3, 0, 4) and (-3, 0, -4). The drill runs past both ends of the ball.
//!
//! solidcore's cylinder has its seam on its +x side, the line (3, 0, z), which would run through
//! the upper pole as well; the case asks for no such coincidence, so the drill is turned a
//! quarter turn about its axis, its seam to (0, 3, z), clear of both poles. (With the seam
//! through the pole the kernel's cut is no closed solid.)
use crate::bench::{cut, rotated};
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    // turning +z about y by asin(3/5) takes it to (3, 0, 4)/5
    let ball = rotated(ball, [0.0; 3], [0.0, 1.0, 0.0], 0.6f64.asin().to_degrees())?;
    let bore = make_cylinder([0.0, 0.0, -6.0], 3.0, 12.0)?;
    let bore = rotated(bore, [0.0; 3], [0.0, 0.0, 1.0], 90.0)?;
    cut(ball, bore)
}
