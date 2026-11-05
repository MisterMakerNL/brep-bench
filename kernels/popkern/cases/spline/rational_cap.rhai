// The region of the xz plane bounded by the axis, the base from (0, −5) to (5√3, −5), and the
// rational quadratic Bézier (5√3, −5), (10√3, 10), (0, 10) of weights 1, ½, 1, turned a whole turn
// about the z axis.
let m = model();
let c = m.profile(xz()).path(0.0, -5.0).line_to(5.0 * sqrt(3.0), -5.0)
    .conic_to([10.0 * sqrt(3.0), 10.0], [0.0, 10.0], 0.5).close().sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
