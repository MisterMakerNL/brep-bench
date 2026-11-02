//! A square-to-round transition. brepkit's loft joins its sections vertex to vertex (resampling
//! the circle and the square to a common polygon), so one loft cannot pair each side of the
//! square with a point of the circle and each corner with a quarter of it. The solid is sewn from
//! its faces instead: the floor; the disc on top, its rim in four quarters; the four flat
//! triangles, each side of the square to the point of the circle nearest it; and the four pieces
//! of oblique cone, each brepkit's loft to a point from the corner to the region between its
//! quarter circle and that quarter's chord, of which the face from the arc to the corner is kept.
use std::f64::consts::FRAC_PI_2;

use brepkit_math::curves::Circle3D;
use brepkit_operations::loft::loft_to_point;
use brepkit_operations::sew::sew_faces;
use brepkit_topology::edge::EdgeCurve;
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::face::FaceSurface;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{chain, polygon, TOL};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // the circle of radius 15 at z = 30, its parameter the angle from +x
    let circle =
        Circle3D::new_with_ref(p(0.0, 0.0, 30.0), v(0.0, 0.0, 1.0), 15.0, v(1.0, 0.0, 0.0))?;
    // anticlockwise about +z: the corner at −45° + 90°k, and the circle's points nearest the
    // sides beside it, at −90° + 90°k and 90°k
    let corners = [
        p(20.0, -20.0, 0.0),
        p(20.0, 20.0, 0.0),
        p(-20.0, 20.0, 0.0),
        p(-20.0, -20.0, 0.0),
    ];
    let near: Vec<_> = (0..4)
        .map(|k| circle.evaluate(-FRAC_PI_2 + FRAC_PI_2 * k as f64))
        .collect();

    let mut faces = vec![polygon(topo, &corners)?];
    faces.push(chain(
        topo,
        &(0..4)
            .map(|k| (near[k], EdgeCurve::Circle(circle.clone())))
            .collect::<Vec<_>>(),
    )?);
    for k in 0..4 {
        // the triangle from the side before this corner to the point nearest that side
        faces.push(polygon(topo, &[corners[(k + 3) % 4], corners[k], near[k]])?);
        // the cone from this corner to its quarter circle
        let segment = chain(
            topo,
            &[
                (near[k], EdgeCurve::Circle(circle.clone())),
                (near[(k + 1) % 4], EdgeCurve::Line),
            ],
        )?;
        let cone = loft_to_point(topo, segment, corners[k])?;
        for face in solid_faces(topo, cone)? {
            if !matches!(topo.face(face)?.surface(), FaceSurface::Plane { .. }) {
                faces.push(face);
            }
        }
    }
    Ok(sew_faces(topo, &faces, TOL)?)
}
