// A cone on the z axis, its base the disc of radius 3 at z = 0 and its apex at (0, 0, 4): its half
// section revolved a full turn about z. Grown by 1.
let m = model();
let c = m.profile(xz()).path(0.0, 0.0).line_to(3.0, 0.0).line_to(0.0, 4.0).close()
    .sketch("cone_section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
c = c.offset_solid(1.0, #{ label: "grown" });
m
