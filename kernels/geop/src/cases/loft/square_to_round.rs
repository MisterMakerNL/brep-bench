//! A square-to-round transition piece from the square −20 ≤ x, y ≤ 20 on the xy plane to the
//! circle of radius 15 on the plane z = 30: each side of the square joined to the point of the
//! circle nearest it by a flat triangle, each corner to the quarter circle between by a piece of
//! an oblique cone.
//!
//! geop's loft joins its profiles by walls running between corresponding points of the two, each
//! piece of the square into a piece of the circle (lofted as they are, it builds walls from each
//! side to a quarter of the circle, a different solid); a side lofted into a single point and a
//! corner into a whole quarter circle it cannot say, with matched points or guides, which must
//! run round both profiles in the same order. Nor does it have a convex hull.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop's loft joins each piece of one profile to a piece of the other, matched in order \
         round both: it cannot loft a side of the square into a single point of the circle (a flat \
         triangle) and a corner into a whole quarter of it (an oblique cone), and it has no convex \
         hull"
        .into())
}
