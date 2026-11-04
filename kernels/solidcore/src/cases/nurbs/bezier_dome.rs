//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, its top the
//! bicubic Bézier patch of control points (20i/3, 20j/3, zᵢⱼ): the patch's face, the floor and
//! the four sides (each bounded by the floor's edge and the cubic the patch's edge draws in its
//! plane), sewn into a solid.
use crate::shared::nurbs::bezier_dome;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    bezier_dome()
}
