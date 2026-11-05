// The lower half of the ball of radius 5 about the origin is a
// quarter disc revolved a full turn about z. Shelled inward to a wall 1 thick, open at its flat
// face, z = 0.
let m = model();
let b = m.profile(xz()).path(0.0, -5.0).arc_to([5.0 * cos(PI() / 4.0), -5.0 * sin(PI() / 4.0)], [5.0, 0.0])
    .line_to(0.0, 0.0).close()
    .sketch("quarter_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
b = b.shell(faces().kind("plane").at_height(0.0), 1.0, #{ label: "hollow" });
m
