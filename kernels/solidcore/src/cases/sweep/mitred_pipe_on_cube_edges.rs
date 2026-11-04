//! A pipe of radius 2, its section square to the path and centred on it, swept along three edges
//! of the cube of side 30, from (0, 0, 0) to (30, 0, 0), to (30, 30, 0), to (30, 30, 30), mitred
//! at its two corners in the planes x + y = 30 and y + z = 30; its ends flat and square to the
//! path.
use crate::bench::{circle, common, fuse, line, polygon};
use crate::shared::sweep::mitred;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let (a, b, c, d) = ([0.0; 3], [30.0, 0.0, 0.0], [30.0, 30.0, 0.0], [30.0, 30.0, 30.0]);
    // The kernel mitres a sweep only along a planar path (`shared::sweep::mitred`), and this one
    // turns out of its plane at its second corner; but each corner joins two runs in a plane.
    // So the first two runs are swept and mitred in the plane z = 0, the second run carried on
    // past the second corner to (30, 40, 0), clear of its mitre plane, and the last two in the
    // plane x = 30; the first sweep is kept on its side of the second corner's mitre plane,
    // y + z <= 30, the second on the other, and the two united.
    let past = [30.0, 40.0, 0.0];
    let first =
        mitred(&[circle(a, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 2.0)], &[line(a, b), line(b, past)])?;
    let second =
        mitred(&[circle(b, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 2.0)], &[line(b, c), line(c, d)])?;
    // the two sides of the plane y + z = 30 about the pipe, as prisms along x on triangles in the
    // plane x = -10 reaching well past it
    let side = |corner: [f64; 2]| -> Result<Body, Refusal> {
        let at = |y: f64, z: f64| [-10.0, y, z];
        let triangle = polygon(&[at(40.0, -10.0), at(corner[0], corner[1]), at(-10.0, 40.0)]);
        Ok(extrude(&triangle, [1.0, 0.0, 0.0], 50.0, 0.0, DraftCorner::default(), true)?)
    };
    let before = common(first, side([-10.0, -10.0])?)?;
    let after = common(second, side([40.0, 40.0])?)?;
    fuse(before, after)
}
