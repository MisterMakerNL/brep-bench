//! What the thicken implementations share: a sheet (a face, or a shell of faces that is not
//! closed) thickened into a solid.
//!
//! kentron has no thickening of a sheet (OCCT's `BRepOffset_MakeOffset` in its thickening mode),
//! so `thickened` makes the same solid from the kernel's other operations: the sheet's faces are
//! moved along their normals by `offset::shape` (which takes a shell or a face as well as a
//! solid), each edge that bounds the sheet is joined to its moved image by a ruled loft (the wall
//! that edge sweeps as the sheet moves), and the solid is the space the sheet, the moved sheet and
//! the walls enclose (`boolean::enclosed_solids`, OCCT's `BOPAlgo_MakerVolume`).
use crate::bench::{face_through, p, v, PICK_TOLERANCE};
use crate::Refusal;
use kentron::geometry::curve::Circle;
use kentron::geometry::ParametricCurve;
use kentron::loft::{self, LoftOptions};
use kentron::math::Point;
use kentron::offset::{self, Join};
use kentron::primitives::Cuboid;
use kentron::topology::{Model, NodeKind, Shape};
use kentron::{boolean, distance, make, sweep};

/// The creased sheet of the angle-bracket cases: the floor from (0, 0, 0) to (10, 8, 0) and the
/// wall from (0, 0, 0) to (0, 8, 10), the polyline (10, 0, 0), (0, 0, 0), (0, 0, 10) swept along
/// +y.
pub fn creased_sheet(m: &mut Model) -> Result<Shape, Refusal> {
    let floor = make::edge_between(m, p(10.0, 0.0, 0.0), p(0.0, 0.0, 0.0))?;
    let wall = make::edge_between(m, p(0.0, 0.0, 0.0), p(0.0, 0.0, 10.0))?;
    let crease = make::wire(m, &[floor, wall])?;
    Ok(sweep::extrude(m, crease, v(0.0, 8.0, 0.0))?.shape)
}

/// The bent sheet of the bent-flange cases: the profile in the xz plane along the line from
/// (−10, 0, 0) to (0, 0, 0), round the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2)
/// and up the line to (2, 0, 12), swept along +y from y = 0 to y = 8.
pub fn bent_sheet(m: &mut Model) -> Result<Shape, Refusal> {
    let floor = make::edge_between(m, p(-10.0, 0.0, 0.0), p(0.0, 0.0, 0.0))?;
    let half = std::f64::consts::FRAC_1_SQRT_2 * 2.0;
    let arc = Circle::arc_through(p(0.0, 0.0, 0.0), p(half, 0.0, 2.0 - half), p(2.0, 0.0, 2.0))
        .map_err(|e| format!("the bend's arc: {e:?}"))?;
    // the edge on the arc's circle itself, so that sweeping it makes a cylinder
    let (first, last) = arc.range();
    let bend = make::edge_from_curve_range(m, arc.basis().clone(), first, last)?;
    let wall = make::edge_between(m, p(2.0, 0.0, 2.0), p(2.0, 0.0, 12.0))?;
    let profile = make::wire(m, &[floor, bend, wall])?;
    Ok(sweep::extrude(m, profile, v(0.0, 8.0, 0.0))?.shape)
}

/// The open tray of the open-box cases: the floor and four walls of the cube from the origin to
/// (10, 10, 10), without its top, as one shell, each face as the cube has it (its normal out of
/// the cube).
pub fn tray(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let shell = m.new_empty(NodeKind::Shell);
    for at in [p(5.0, 5.0, 0.0), p(5.0, 0.0, 5.0), p(5.0, 10.0, 5.0), p(0.0, 5.0, 5.0), p(10.0, 5.0, 5.0)] {
        let face = face_through(m, cube, at)?;
        m.add(shell, face)?;
    }
    Ok(shell)
}

/// The edges that bound `sheet`: each used once by its faces (not shared by two faces, nor a
/// seam where one face closes on itself).
pub fn free_edges(m: &Model, sheet: Shape) -> Vec<Shape> {
    let mut uses: Vec<(Shape, usize)> = Vec::new();
    for face in m.unique_subshapes(sheet, NodeKind::Face) {
        for edge in m.explore(face, NodeKind::Edge).collect::<Vec<_>>() {
            match uses.iter_mut().find(|(e, _)| e.node == edge.node) {
                Some((_, n)) => *n += 1,
                None => uses.push((edge, 1)),
            }
        }
    }
    uses.into_iter().filter(|&(e, n)| n == 1 && !m.is_degenerate(e)).map(|(e, _)| e).collect()
}

/// The point halfway along `edge`'s parameters.
fn middle(m: &Model, edge: Shape) -> Result<Point, Refusal> {
    let (curve, first, last) = m.edge_curve(edge).ok_or("an edge of the sheet without a curve")?;
    Ok(curve.point(0.5 * (first + last)))
}

/// The solid between `sheet` and its faces moved `distance` along their normals (to the other
/// side for a negative distance), the moved faces joined as `join` says where they part.
pub fn thickened(m: &mut Model, sheet: Shape, distance: f64, join: Join) -> Result<Shape, Refusal> {
    let bounds = free_edges(m, sheet);
    let moved = offset::shape(m, sheet, distance, join)?;
    let images = free_edges(m, moved);
    if images.len() != bounds.len() {
        // The rounds joining moved faces that part at a crease have ends of their own, which no
        // edge of the sheet sweeps.
        return Err("kentron has no thickening of a sheet, and its offset of the sheet joined by arcs rounds the creases with faces whose ends no edge of the sheet sweeps".into());
    }
    let mut faces = vec![sheet, moved];
    for edge in bounds {
        // The edge's image: the free edge of the moved sheet nearest the edge's middle.
        let at = middle(m, edge)?;
        let at = make::vertex(m, at);
        let mut nearest: Option<(f64, Shape)> = None;
        for &image in &images {
            let d = distance::distance(m, at, image)?.value;
            if nearest.is_none_or(|(best, _)| d < best) {
                nearest = Some((d, image));
            }
        }
        let (_, image) = nearest.ok_or("the moved sheet has no free edge")?;
        let from = make::wire(m, &[edge])?;
        let to = make::wire(m, &[image])?;
        let wall = loft::loft(m, &[from, to], &LoftOptions { ruled: true, ..Default::default() })?;
        faces.push(wall.shape);
    }
    // Of the pieces of space they enclose, the ones the moved sheet bounds: a closed sheet also
    // encloses the space inside it on its own, which lies on no side of the thickening.
    let enclosed = boolean::enclosed_solids(m, &faces)?;
    let mut kept = Vec::new();
    for solid in m.unique_subshapes(enclosed, NodeKind::Solid) {
        if distance::distance(m, solid, moved)?.value < PICK_TOLERANCE {
            kept.push(solid);
        }
    }
    match kept[..] {
        [] => Err("the sheet, its offset and the walls between them enclose no solid".into()),
        [solid] => Ok(solid),
        _ => {
            let compound = m.new_empty(NodeKind::Compound);
            m.add_all(compound, &kept)?;
            Ok(compound)
        }
    }
}
