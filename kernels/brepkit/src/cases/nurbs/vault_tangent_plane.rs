//! A block on the square −12 ≤ x, y ≤ 12 under a weighted barrel vault, less everything above
//! z = 16. The vault is brepkit's rational NURBS surface on the case's six control points and
//! weights (degree 1 along x on (0, 0, 1, 1), degree 2 along y on (0, 0, 0, 1, 1, 1)); its face is
//! bounded by its two arched edges (the rational quadratics on its rows of control points) and its
//! two straight ones, each side the flat face between the floor and one of them, all sewn. The cut
//! is a box whose floor is the plane z = 16, reaching past the block on every other side.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{block_under, nurbs, rational_bezier, surface};
use crate::solids::{cut, moved, p};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // the vault's rows of control points, at x = −12 and x = 12, each one weighted quadratic arc
    let row = |x: f64| vec![p(x, -12.0, 10.0), p(x, 0.0, 26.0), p(x, 12.0, 10.0)];
    let weights = [1.0, 0.6, 1.0];
    let vault = surface(
        1,
        2,
        &[0.0, 0.0, 1.0, 1.0],
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![row(-12.0), row(12.0)],
        vec![weights.to_vec(), weights.to_vec()],
    )?;
    let line = |a, b| nurbs(1, &[0.0, 0.0, 1.0, 1.0], &[a, b], &[1.0, 1.0]);
    let block = block_under(
        topo,
        vault,
        [
            rational_bezier(&row(-12.0), &weights)?,
            rational_bezier(&row(12.0), &weights)?,
            line(p(-12.0, -12.0, 10.0), p(12.0, -12.0, 10.0))?,
            line(p(-12.0, 12.0, 10.0), p(12.0, 12.0, 10.0))?,
        ],
    )?;
    let lid = make_box(topo, 40.0, 40.0, 20.0)?;
    let lid = moved(topo, lid, -20.0, -20.0, 16.0)?;
    Ok(cut(topo, block, lid)?)
}
