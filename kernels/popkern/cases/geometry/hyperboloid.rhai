// The branch x²/16 − z²/36 = 1, x > 0, from (4√2, −6) to (4√2, 6) as one exact conic: its end
// tangents meet at (2√2, 0), and the weight √2 puts it through the waist (4, 0). Closed along the
// two ends and the axis, turned a whole turn about z.
let m = model();
let r = 4.0 * sqrt(2.0);
let h = m.profile(xz()).path(0.0, -6.0).line_to(r, -6.0)
    .conic_to([2.0 * sqrt(2.0), 0.0], [r, 6.0], sqrt(2.0)).line_to(0.0, 6.0).close()
    .sketch("section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
