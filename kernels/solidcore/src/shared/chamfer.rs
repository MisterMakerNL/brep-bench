//! What the solidcore implementations of the chamfer cases share.
use crate::bench::{edge_through, fuse, polygon, reloaded};
use crate::Refusal;
use solidcore_ops::split::{split_body, Tool};
use solidcore_ops::sweep::loft_to_point;
use solidcore_ops::{chamfer_sides, edge_faces_along, Body, ChamferSecond};
use solidcore_record::brep::Surface;

/// The octahedron |x| + |y| + |z| ≤ 10: the square through its four corners in the plane z = 0,
/// lofted to a point at each of its other two corners, (0, 0, 10) and (0, 0, −10), the two
/// pyramids united.
pub fn octahedron() -> Result<Body, Refusal> {
    let square =
        polygon(&[[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [-10.0, 0.0, 0.0], [0.0, -10.0, 0.0]]);
    let upper = loft_to_point(&square, [0.0, 0.0, 10.0])?;
    let lower = loft_to_point(&square, [0.0, 0.0, -10.0])?;
    fuse(upper, lower)
}

/// The edges of `body` at record indices `edges` (read back in record order,
/// [`crate::bench::reloaded`]) chamfered with `distance` on both of their faces: ops' chamfer by two
/// distances, the first along either face of each edge, the second the same. (The equal
/// chamfer of [`solidcore_ops::blend`] refused an edge of the octahedron alone, at its corners
/// of four edges, as invalid input, where this one took it.)
pub fn chamfer_equal(body: &Body, edges: &[usize], distance: f64) -> Result<(), Refusal> {
    let mut along = Vec::new();
    for &e in edges {
        // either face will do, the distances being equal: the first
        let [(face, _), _] = edge_faces_along(body, e, [0.0, 0.0, 1.0])?;
        along.push(face);
    }
    chamfer_sides(body, edges, &along, distance, ChamferSecond::Distance(distance))?;
    Ok(())
}

/// `body` with the edges through `points` chamfered one after another at `distance` on both
/// faces, each its own chamfer, ending on the faces at its ends, the chamfers made before it
/// among them. Each edge is picked again after the chamfers before it.
pub fn chamfer_one_by_one(body: Body, points: &[[f64; 3]], distance: f64) -> Result<Body, Refusal> {
    let mut body = body;
    for &p in points {
        let (b, order) = reloaded(&body)?;
        let edge = edge_through(&b, &order, p)?;
        chamfer_equal(&b, &[edge], distance)?;
        body = b;
    }
    Ok(body)
}

/// `body` less what lies in front of the plane through `point` with normal `normal`: the part
/// behind the plane when ops' Split Body cuts the body in two by it.
pub fn cut_off(body: &Body, point: [f64; 3], normal: [f64; 3]) -> Result<Body, Refusal> {
    let len = |a: [f64; 3]| (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    let unit = |a: [f64; 3]| a.map(|x| x / len(a));
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
    };
    let n = unit(normal);
    // any direction in the plane, then the one square to it with u × v = n
    let other = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = unit(cross(other, n));
    let v = cross(n, u);
    let tool = Tool {
        surface: Surface::Plane { origin: point, u_axis: u, v_axis: v },
        domain: [[-100.0, 100.0], [-100.0, 100.0]],
    };
    match split_body(body, &tool)? {
        Some((behind, _)) => Ok(behind),
        None => Err("the plane leaves one side of the body empty".into()),
    }
}
