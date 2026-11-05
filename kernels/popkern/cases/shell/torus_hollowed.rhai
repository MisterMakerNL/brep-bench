// A torus about z (middle circle 6, tube 2), the tube's section revolved a full turn about z.
// Hollowed to a wall 0.5 thick with no opening: no face is removed, so the shell leaves a closed
// cavity.
let m = model();
let t = m.profile(xz()).circle(2.0).at(6.0, 0.0).sketch("tube")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
t = t.shell(faces().kind("plane").allow_empty(), 0.5, #{ label: "hollow" });
m
