//! A cube of side 10 from the origin to (10, 10, 10), all twelve edges chamfered with distance 2
//! on both faces, the three chamfers at each corner closed by a flat triangle: the cube cut by
//! the twelve chamfer planes (x + y ≤ 18 and the like) and by the eight corner planes
//! (x + y + z ≤ 26 and the like).
//!
//! solidcore's chamfer of all twelve edges at once runs the chamfers on until they meet, the
//! three at each corner in a point (`chamfer/cube-all-edges-mitred`), and closes no corner with
//! a face of its own; so each corner is then cut flat by its plane, through the three corners of
//! the squares nearest it, with ops' Split Body.
use crate::bench::{all_edges, reloaded};
use crate::shared::chamfer::cut_off;
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (cube, order) = reloaded(&make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?)?;
    blend(&cube, Blend::Chamfer, &all_edges(&order), &[2.0], &[], Corners::Filled)?;
    let mut solid = cube;
    for sx in [-1.0, 1.0] {
        for sy in [-1.0, 1.0] {
            for sz in [-1.0, 1.0] {
                // the corner (5 ± 5, 5 ± 5, 5 ± 5) and the corner of the square on its face
                // z = 5 ± 5 nearest it, (5 ± 3, 5 ± 3, 5 ± 5), through which its plane runs
                let near = [5.0 + 3.0 * sx, 5.0 + 3.0 * sy, 5.0 + 5.0 * sz];
                solid = cut_off(&solid, near, [sx, sy, sz])?;
            }
        }
    }
    Ok(solid)
}
