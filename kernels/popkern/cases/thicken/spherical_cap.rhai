// The cap of the sphere of radius 10 where z >= 6, thickened by 2 away from the centre. popkern
// scripts make sheets from solids: the cap's solid (its half section revolved about z) with its flat
// base removed.
let m = model();
let c = m.profile(xz()).path(0.0, 6.0).line_to(8.0, 6.0)
    .arc_to([4.47213595499958, 8.94427190999916], [0.0, 10.0]).line_to(0.0, 6.0).close()
    .sketch("cap_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
c = c.remove_faces(faces().kind("plane"), #{ label: "cap" });
c = c.thicken(2.0, #{ side: "outward", label: "thick" });
m
