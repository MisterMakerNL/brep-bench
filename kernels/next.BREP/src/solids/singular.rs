//! Helpers shared by the tangent and singular cases: rigid motions and half-space cuts by the
//! kernel's own split.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_box_brep, split_solid_by_plane, transform_brep, AffineTransform,
    BooleanOperation, BooleanOptions, BrepSolid, Vec3,
};

/// `solid` turned by `angle` (radians, right-handed) about the line through `through` along
/// `axis`, then moved by `shift`.
pub fn turned(
    solid: &BrepSolid,
    through: Vec3,
    axis: Vec3,
    angle: f64,
    shift: Vec3,
) -> Result<BrepSolid, String> {
    let k = axis.normalized()?;
    let column = |e: Vec3| e.rotate_about(k, angle);
    let (cx, cy, cz) = (column(Vec3::X), column(Vec3::Y), column(Vec3::Z));
    // x' = R(x − c) + c + shift
    let t = through.add(shift).sub(
        cx.scale(through.x).add(cy.scale(through.y)).add(cz.scale(through.z)),
    );
    #[rustfmt::skip]
    let m = AffineTransform::new([
        cx.x, cy.x, cz.x, t.x,
        cx.y, cy.y, cz.y, t.y,
        cx.z, cy.z, cz.z, t.z,
        0.0, 0.0, 0.0, 1.0,
    ])?;
    transform_brep(solid, m, false)
}

/// A block of half-width `reach` lying on the side of the plane through `point` toward which
/// `normal` points, one of its faces in that plane: a half-space, as far as a solid of that size can
/// tell. The block is made square to z and turned rigidly so that +z goes to `normal`.
pub fn half_space(point: Vec3, normal: Vec3, reach: f64) -> Result<BrepSolid, String> {
    let n = normal.normalized()?;
    let block = make_box_brep(p(-reach, -reach, 0.0), 2.0 * reach, 2.0 * reach, 2.0 * reach)?;
    let axis = Vec3::Z.cross(n);
    let (axis, angle) = if axis.length() <= 1e-15 {
        (Vec3::X, if n.z > 0.0 { 0.0 } else { std::f64::consts::PI })
    } else {
        (axis, axis.length().atan2(n.z))
    };
    turned(&block, p(0.0, 0.0, 0.0), axis, angle, point)
}

/// What of `solid` lies on the side of the plane through `point` toward which `normal` points:
/// its common part with a block reaching `reach` past the plane's foot on that side.
pub fn keep_toward(solid: &BrepSolid, point: Vec3, normal: Vec3, reach: f64) -> Result<BrepSolid, Refusal> {
    let block = half_space(point, normal, reach)?;
    Ok(boolean_operation(solid, &block, BooleanOperation::Intersect, &BooleanOptions::default())?)
}

/// The same, by the kernel's split of a solid by a plane.
pub fn split_toward(solid: &BrepSolid, point: Vec3, normal: Vec3) -> Result<BrepSolid, String> {
    Ok(split_solid_by_plane(solid, point, normal)?.1)
}

/// What of `solid` lies on the side of the plane through `point` away from which `normal` points:
/// `solid` less a block reaching `reach` past the plane's foot on the `normal` side.
pub fn drop_toward(solid: &BrepSolid, point: Vec3, normal: Vec3, reach: f64) -> Result<BrepSolid, Refusal> {
    let block = half_space(point, normal, reach)?;
    Ok(boolean_operation(solid, &block, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
