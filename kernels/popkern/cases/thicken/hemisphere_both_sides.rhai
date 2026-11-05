// A hemispherical sheet of radius 5 thickened by 1 to each side (2 in all).
let m = model();
// The sheet is made from a solid: the half ball of radius 5 where z >= 0 (a quarter disc
// revolved about z), with its flat face removed.
let h = m.profile(xz()).path(0.0, 0.0).line_to(5.0, 0.0).arc_to([3.5355339059327378, 3.5355339059327378], [0.0, 5.0])
    .line_to(0.0, 0.0).close()
    .sketch("quarter_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
h = h.remove_faces(faces().kind("plane"), #{ label: "dome" });
h = h.thicken(2.0, #{ side: "both", label: "thick" });
m
