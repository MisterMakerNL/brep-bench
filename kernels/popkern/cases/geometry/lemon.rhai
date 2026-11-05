// A lemon: the segment of the disc of radius 5 centred at (-3, 0, 0) in the xz plane cut off by the
// z axis, from (0, 0, -4) out to (2, 0, 0) and back to (0, 0, 4), turned a whole turn about the
// z axis.
let m = model();
m.profile(xz()).path(0.0, -4.0).arc_to([2.0, 0.0], [0.0, 4.0]).line_to(0.0, -4.0).close().sketch("segment")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
