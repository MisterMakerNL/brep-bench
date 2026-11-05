// The rational quadratic B-spline (0, −10), (6, −10), (6, 0), (6, 10), (0, 10), weights
// 1, √2/2, 1, √2/2, 1, knots (0, 0, 0, ½, ½, 1, 1, 1), closed along the z axis and turned a whole
// turn about it.
let m = model();
let w = sqrt(2.0) / 2.0;
let s = m.profile(xz()).path(0.0, -10.0)
    .spline_to([[6.0, -10.0], [6.0, 0.0], [6.0, 10.0], [0.0, 10.0]],
        #{ degree: 2, weights: [1.0, w, 1.0, w, 1.0], knots: [0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0] })
    .close().sketch("section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
