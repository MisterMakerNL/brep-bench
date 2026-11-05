// The periodic uniform quadratic B-spline over the square (13, 3), (7, 3), (7, −3), (13, −3) of
// the xz plane, turned a whole turn about the z axis.
let m = model();
let r = m.profile(xz()).spline([[13.0, 3.0], [7.0, 3.0], [7.0, -3.0], [13.0, -3.0]], #{ degree: 2 })
    .sketch("section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
