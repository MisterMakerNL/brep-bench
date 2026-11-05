// A torus about the z axis, its middle circle of radius 5 and its tube of radius 4.99: the disc of
// radius 4.99 centred at (5, 0, 0) in the xz plane, turned a whole turn about the z axis.
let m = model();
m.profile(xz()).circle(4.99).at(5.0, 0.0).sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
