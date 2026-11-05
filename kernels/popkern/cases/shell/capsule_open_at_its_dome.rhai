// A capsule's end, a cylinder of radius 5 from z = 0 to 10 capped by the half ball of radius 5 about
// (0, 0, 10): its half section revolved a full turn about z. Shelled inward to a wall 1 thick,
// open at its hemisphere.
let m = model();
let c = m.profile(xz())
    .path(0.0, 0.0).line_to(5.0, 0.0).line_to(5.0, 10.0)
    .arc_to([5.0 * cos(PI() / 4.0), 10.0 + 5.0 * sin(PI() / 4.0)], [0.0, 15.0]).close()
    .sketch("capsule_section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
c = c.shell(faces().kind("sphere"), 1.0, #{ label: "hollow" });
m
