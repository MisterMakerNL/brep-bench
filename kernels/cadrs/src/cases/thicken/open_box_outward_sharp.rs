//! A sheet: five faces of the cube from the origin to (10, 10, 10), its floor at z = 0 and its four
//! walls, with no top (an open tray), thickened by 1 out of the cube with its edges kept sharp: the
//! moved faces extended until they meet.
//!
//! cadrs's Thicken of the floor and walls shells the solid they bound with its corners rounded (a
//! cylinder round each edge, a ball at each corner), with no option to keep them sharp. So the
//! tray is built with cadrs's Shell instead, which has that option: the cube (a square on the Top
//! plane extruded 10) shelled 1 outward with its top removed, the walls' corners extended sharp.
use crate::bench::{block, face_at, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, ShellSpec};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let top = face_at(k, cube, p3(5.0, 5.0, 10.0))?;
    let spec =
        ShellSpec { remove: vec![top], thickness: 1.0, outward: true, hollow: false, sharp: true };
    let r = k.shell_with(cube, &spec)?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the shell made no body"))
}
