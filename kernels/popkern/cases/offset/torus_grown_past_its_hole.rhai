// A torus about the z axis: a circle of radius 2 centred 4 out along x in the xz plane, revolved a
// full turn about z. Grown by 3, past its hole.
let m = model();
let t = m.profile(xz()).circle(2.0).at(4.0, 0.0).sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
t = t.offset_solid(3.0, #{ label: "grown" });
m
