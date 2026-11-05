// The side of the cone frustum about z, radius 6 at z = 0 to 3 at z = 4, thickened by 1 along its
// normal away from the axis. popkern scripts make sheets from solids: the frustum, revolved from its
// half section, with its two flat ends removed.
let m = model();
let c = m.profile(xz()).path(0.0, 0.0).line_to(6.0, 0.0).line_to(3.0, 4.0).line_to(0.0, 4.0).close()
    .sketch("frustum_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
c = c.remove_faces(faces().kind("plane"), #{ label: "side" });
c = c.thicken(1.0, #{ side: "outward", label: "thick" });
m
