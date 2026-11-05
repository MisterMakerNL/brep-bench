// A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8): its
// half section revolved a full turn about z. Shelled inward to a wall 1 thick, open at its base.
let m = model();
let c = m.profile(xz()).path(0.0, 0.0).line_to(6.0, 0.0).line_to(0.0, 8.0).close()
    .sketch("cone_section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
c = c.shell(faces().kind("plane").at_height(0.0), 1.0, #{ label: "hollow" });
m
