// The half ellipse x²/36 + z²/9 ≤ 1, x ≥ 0, of the xz plane (two exact conic quarters, weight
// √2/2, shoulders at the bounding box's corners), turned a whole turn about the z axis.
let m = model();
let w = 0.7071067811865476;
let s = m.profile(xz()).path(0.0, -3.0).conic_to([6.0, -3.0], [6.0, 0.0], w)
    .conic_to([6.0, 3.0], [0.0, 3.0], w).close().sketch("half")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
