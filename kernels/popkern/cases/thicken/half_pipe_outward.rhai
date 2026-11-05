// A half-pipe sheet, the half of the cylinder of radius 5 about z where y >= 0, thickened by 1 away
// from its axis.
let m = model();
// popkern scripts make sheets from solids: the half cylinder of radius 5 about z where y >= 0, from
// z = 0 to 10, with its three flat faces removed.
let p = m.profile(xy()).path(5.0, 0.0).arc_to([0.0, 5.0], [-5.0, 0.0]).line_to(5.0, 0.0).close()
    .sketch("half_disc").extrude(blind(10.0));
p = p.remove_faces(faces().kind("plane"), #{ label: "half_pipe" });
p = p.thicken(1.0, #{ side: "outward", label: "thick" });
m
