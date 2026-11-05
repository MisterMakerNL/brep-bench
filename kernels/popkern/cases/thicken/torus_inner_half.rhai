// The inner half of the torus with tube radius 2 about a middle circle of radius 6 (the points no
// farther than 6 from the z axis), thickened by 1 away from the middle circle. popkern scripts make
// sheets from solids: the half tube (its half section revolved about z) with its cylindrical face,
// radius 6, removed.
let m = model();
let t = m.profile(xz()).path(6.0, -2.0).arc_to([4.0, 0.0], [6.0, 2.0]).line_to(6.0, -2.0).close()
    .sketch("half_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
t = t.remove_faces(faces().kind("cylinder"), #{ label: "saddle" });
t = t.thicken(1.0, #{ side: "outward", label: "thick" });
m
