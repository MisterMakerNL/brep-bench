// The parabola x² = 8z from its vertex to (8, 8) (a conic of weight 1: the quadratic Bézier with
// shoulder (4, 0)), closed across the top and down the axis, turned a whole turn about z.
let m = model();
let p = m.profile(xz()).path(0.0, 0.0).conic_to([4.0, 0.0], [8.0, 8.0], 1.0).line_to(0.0, 8.0)
    .close().sketch("section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
