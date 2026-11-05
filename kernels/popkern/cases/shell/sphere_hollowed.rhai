// The ball is a half disc of radius 5 revolved a full turn about z. Hollowed to a
// wall 1 thick with no opening: no face is removed, so the shell leaves a closed cavity.
let m = model();
let b = m.profile(xz()).path(0.0, -5.0).arc_to([5.0, 0.0], [0.0, 5.0]).line_to(0.0, -5.0).close()
    .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
b = b.shell(faces().kind("plane").allow_empty(), 1.0, #{ label: "hollow" });
m
