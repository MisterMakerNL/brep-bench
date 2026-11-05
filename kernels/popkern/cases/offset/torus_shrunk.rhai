// A torus about the z axis: a circle of radius 3 centred 10 out along x in the xz plane, revolved a
// full turn about z. Shrunk by 1.5.
let m = model();
let t = m.profile(xz()).circle(3.0).at(10.0, 0.0).sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
t = t.offset_solid(-1.5, #{ label: "shrunk" });
m
