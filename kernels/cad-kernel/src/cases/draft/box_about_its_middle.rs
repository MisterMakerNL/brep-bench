//! The box from the origin to (20, 12, 10), its four sides drafted 5° inward about z = 5.
use crate::sketch::polygon;
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

const DRAFT_DEGREES: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // cad-kernel's drafted prism tapers away from its profile's plane both ways, so a section at
    // z = 5 would narrow below it too; the sides lean one way throughout, so the profile is the
    // floor, the box's rectangle outset 5·tan 5° as the case says, tapered up from there
    let d = 5. * DRAFT_DEGREES.to_radians().tan();
    let floor = polygon(&[[-d, -d, 0.], [20. + d, -d, 0.], [20. + d, 12. + d, 0.], [-d, 12. + d, 0.]]);
    drafted(&floor, 0., 10., DRAFT_DEGREES.to_radians())
}
