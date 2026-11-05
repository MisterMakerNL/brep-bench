// A torus about z (middle circle 6, tube 2) with everything where y < 0 cut away by a block, then
// shelled inward to a wall 0.5 thick, open at both its flat faces in the plane y = 0.
let m = model();
let t = m.profile(xz()).circle(2.0).at(6.0, 0.0).sketch("tube")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
let half = m.tool(|m| m.profile(xy_at(-5.0)).rect(30.0, 15.0).at(0.0, -7.5)
    .sketch("half").extrude(blind(10.0)));
t = t.subtract(half);
t = t.shell(faces().kind("plane").at_height(0.0), 0.5, #{ label: "hollow" });
m
