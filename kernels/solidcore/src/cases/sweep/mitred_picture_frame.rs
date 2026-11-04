//! A picture frame: the moulding whose section, by s outward from the path and z, is the polygon
//! through (0, 2), (3, 2), (3, 0), (8, 0), (8, 6), (0, 6), swept round the closed rectangle with
//! corners (±30, ±20, 0), each corner mitred in the vertical plane along its diagonal.
use crate::bench::{line, polygon};
use crate::shared::sweep::mitred;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the section where the path starts, half way along the side y = -20: the point (s, z) is
    // at (0, -20 - s, z)
    let at = |s: f64, z: f64| [0.0, -20.0 - s, z];
    let section = polygon(&[
        at(0.0, 2.0),
        at(3.0, 2.0),
        at(3.0, 0.0),
        at(8.0, 0.0),
        at(8.0, 6.0),
        at(0.0, 6.0),
    ]);
    // The kernel's sweep along a planar path carries the section by the middle of its box (s = 4,
    // z = 3), putting that on the path; so the path it is given is the one that point runs
    // along: the rectangle 4 further out, corners (±34, ±24), at z = 3. Its corners' diagonals
    // are the frame's mitre planes. It runs clockwise seen from above, from the middle of the
    // side y = -24 along -x and back to the start: the kernel sets the section down in its own
    // frame of the path's start (x along the path, y the z axis crossed with it), and along +x
    // that frame turns the section over, its rebate on the outside.
    let corners = [
        [0.0, -24.0, 3.0],
        [-34.0, -24.0, 3.0],
        [-34.0, 24.0, 3.0],
        [34.0, 24.0, 3.0],
        [34.0, -24.0, 3.0],
    ];
    let mut path: Vec<_> = corners.windows(2).map(|w| line(w[0], w[1])).collect();
    path.push(line(corners[4], corners[0]));
    mitred(&section, &path)
}
