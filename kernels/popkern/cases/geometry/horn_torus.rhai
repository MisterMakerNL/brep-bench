// A horn torus about the z axis: the disc of radius 5 centred at (5, 0, 0) in the xz plane, which
// touches the axis at the origin, turned a whole turn about the z axis.
let m = model();
m.profile(xz()).circle(5.0).at(5.0, 0.0).sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
