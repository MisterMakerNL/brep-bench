//! The angle sheet, a floor (the rectangle in the xy plane from the origin to (10, 8, 0)) and a
//! wall (the rectangle in the yz plane from the origin to (0, 8, 10)) sharing the crease along
//! y, thickened by 1 to the inside of the crease (+z and +x), the moved faces trimmed where they
//! cross.
use crate::shared::thicken::{angle_sheet, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the floor's normal is +z and the wall's +x: along them
    thickened(angle_sheet()?, 1.0, false)
}
