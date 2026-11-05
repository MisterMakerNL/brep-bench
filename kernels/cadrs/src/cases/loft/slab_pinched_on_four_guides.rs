//! A slab lofted through rectangles centered on the z axis, sides parallel to x and y, from 24 ×
//! 6.4 at z = 0 to 6 × 24.4 at z = 24, its corners on four guides: the cubic Bézier curve with
//! control points (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19), (3, 12.2, 24) and its mirror images
//! in the xz plane, the yz plane and both. Each wall is swept by a horizontal segment between two
//! neighboring guides; the ends are flat.
//!
//! cadrs's solid Loft takes no guides, so the slab is built as its surfacing would make it: each
//! wall a surface Loft through curves from the rectangle's side at z = 0 to its side at z = 24
//! along the two guides through its corners, each end the Fill of its rectangle, the six sheets
//! sewn into a solid.
use crate::bench::{line, p2, p3, xy};
use crate::shared::loft::{flat, sewn, surface_loft};
use crate::shared::sweep::{bezier, on};
use crate::Refusal;
use cadrs_kernel::{BodyId, CurveSection, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the guide through the corner with x and y of signs (sx, sy)
    let guide = |sx: f64, sy: f64| {
        bezier(&[
            p3(12.0 * sx, 3.2 * sy, 0.0),
            p3(18.0 * sx, -2.8 * sy, 8.0),
            p3(15.0 * sx, 0.2 * sy, 19.0),
            p3(3.0 * sx, 12.2 * sy, 24.0),
        ])
    };
    // the corners at z = 0 and z = 24, counter-clockwise from (+, −)
    let signs = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)];
    let foot: Vec<(f64, f64)> = signs.iter().map(|&(sx, sy)| (12.0 * sx, 3.2 * sy)).collect();
    let head: Vec<(f64, f64)> = signs.iter().map(|&(sx, sy)| (3.0 * sx, 12.2 * sy)).collect();
    let mut sheets = vec![flat(k, 0.0, &foot)?, flat(k, 24.0, &head)?];
    for i in 0..4 {
        let j = (i + 1) % 4;
        let side = |z: f64, c: &[(f64, f64)]| {
            CurveSection::Wire(vec![on(xy(z), line(p2(c[i].0, c[i].1), p2(c[j].0, c[j].1)))])
        };
        let guides = vec![vec![guide(signs[i].0, signs[i].1)], vec![guide(signs[j].0, signs[j].1)]];
        sheets.push(surface_loft(k, vec![side(0.0, &foot), side(24.0, &head)], guides)?);
    }
    sewn(k, &sheets)
}
