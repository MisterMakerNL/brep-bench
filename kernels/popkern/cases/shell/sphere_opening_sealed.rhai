// popkern has no ball primitive: the part of the ball of radius 5 about the origin below z = 4.5 is
// its half section revolved a full turn about z. Shelled inward to a wall 1 thick, open at its flat
// face, z = 4.5.
let m = model();
let h = 4.5;
let b = m.profile(xz()).path(0.0, -5.0).arc_to([5.0, 0.0], [sqrt(25.0 - h * h), h]).line_to(0.0, h)
    .close()
    .sketch("cap_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
b = b.shell(faces().kind("plane").at_height(h), 1.0, #{ label: "hollow" });
m
