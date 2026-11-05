// The sphere of radius 5 as a closed sheet, thickened by 1 outward into a hollow ball. A closed body
// thickens into a hollow one, so the ball itself (a half disc revolved about z) stands for its skin.
let m = model();
let s = m.profile(xz()).path(0.0, -5.0).arc_to([5.0, 0.0], [0.0, 5.0]).line_to(0.0, -5.0).close()
    .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
s = s.thicken(1.0, #{ side: "outward", label: "thick" });
m
