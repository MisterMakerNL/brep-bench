// A spindle torus about the z axis, its middle circle of radius 3 and its tube of radius 5: what the
// disc of radius 5 centred at (3, 0, 0) in the xz plane fills as it turns a whole turn about the
// z axis, counted once. That is the turn of the disc's part with x >= 0, bounded by its outer arc
// from (0, 0, -4) through (8, 0, 0) to (0, 0, 4).
let m = model();
m.profile(xz()).path(0.0, -4.0).arc_to([8.0, 0.0], [0.0, 4.0]).line_to(0.0, -4.0).close()
    .sketch("section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
