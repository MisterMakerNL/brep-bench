//! A box from the origin to (20, 10, 6), the four edges around its top face chamfered unequally:
//! distance 1 across the top face and 2 down the side face.
use crate::bench::{edges_through_each, face_through, reloaded};
use crate::Refusal;
use solidcore_ops::{chamfer_sides, make_box, Body, ChamferSecond};

pub fn build() -> Result<Body, Refusal> {
    let (box_, order) = reloaded(&make_box([0.0, 0.0, 0.0], [20.0, 10.0, 6.0])?)?;
    let top = face_through(&box_, [10.0, 5.0, 6.0])?;
    let rim = edges_through_each(
        &box_,
        &order,
        &[[10.0, 0.0, 6.0], [20.0, 5.0, 6.0], [10.0, 10.0, 6.0], [0.0, 5.0, 6.0]],
    )?;
    // the first distance along the top face, the second down the side
    let along = vec![top; rim.len()];
    chamfer_sides(&box_, &rim, &along, 1.0, ChamferSecond::Distance(2.0))?;
    Ok(box_)
}
