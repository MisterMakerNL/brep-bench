// The branch z²/9 − x²/16 = 1, z > 0, from its vertex (0, 3) to (4√3, 6) as one exact conic: its
// end tangents meet at (4/√3, 3), and the weight √6/2 puts it on the hyperbola mid-span (where the
// line from the chord's midpoint to that shoulder meets it). Closed across the top and down the
// axis, turned a whole turn about z.
let m = model();
let h = m.profile(xz()).path(0.0, 3.0)
    .conic_to([4.0 / sqrt(3.0), 3.0], [4.0 * sqrt(3.0), 6.0], sqrt(6.0) / 2.0).line_to(0.0, 6.0)
    .close().sketch("section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
