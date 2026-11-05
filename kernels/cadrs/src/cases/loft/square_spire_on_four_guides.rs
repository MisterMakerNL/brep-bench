//! A square spire lofted from the square 18 × 18 centered on the z axis at z = 0 to the apex
//! (0, 0, 24), its sections squares whose corners lie on four ridges: the cubic Bézier curve with
//! control points (9, 9, 0), (15, 15, 8), (12, 12, 19), (0, 0, 24) and its mirror images in the
//! xz plane, the yz plane and both. Each wall is swept by a horizontal segment between two
//! neighboring ridges; the floor is flat.
//!
//! cadrs's solid Loft takes no guides, so the spire is built as its surfacing would make it:
//! each wall a surface Loft through curves from the square's side at z = 0 to the apex along the
//! two ridges through its corners, the floor the Fill of the square, the five sheets sewn into a
//! solid.
use crate::bench::{line, p2, p3, xy};
use crate::shared::loft::{flat, sewn, surface_loft};
use crate::shared::sweep::{bezier, on};
use crate::Refusal;
use cadrs_kernel::{BodyId, CurveSection, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let apex = p3(0.0, 0.0, 24.0);
    // the ridge through the corner with x and y of signs (sx, sy)
    let ridge = |sx: f64, sy: f64| {
        bezier(&[p3(9.0 * sx, 9.0 * sy, 0.0), p3(15.0 * sx, 15.0 * sy, 8.0), p3(12.0 * sx, 12.0 * sy, 19.0), apex])
    };
    // the corners at z = 0, counter-clockwise from (+, −)
    let signs = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)];
    let foot: Vec<(f64, f64)> = signs.iter().map(|&(sx, sy)| (9.0 * sx, 9.0 * sy)).collect();
    let mut sheets = vec![flat(k, 0.0, &foot)?];
    for i in 0..4 {
        let j = (i + 1) % 4;
        let side = on(xy(0.0), line(p2(foot[i].0, foot[i].1), p2(foot[j].0, foot[j].1)));
        let guides = vec![vec![ridge(signs[i].0, signs[i].1)], vec![ridge(signs[j].0, signs[j].1)]];
        sheets.push(surface_loft(k, vec![CurveSection::Wire(vec![side]), CurveSection::Point(apex)], guides)?);
    }
    sewn(k, &sheets)
}
