// The region of the xz plane bounded by the axis, the base from (0, 0) to (9, 0), and the cubic
// Bézier (9, 0), (12, 4), (9, 10), (0, 10), turned a whole turn about the z axis.
let m = model();
let d = m.profile(xz()).path(0.0, 0.0).line_to(9.0, 0.0)
    .spline_to([[12.0, 4.0], [9.0, 10.0], [0.0, 10.0]], #{ degree: 3 }).close().sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
