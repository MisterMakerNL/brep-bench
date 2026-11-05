// A half-pipe sheet, the half of the cylinder of radius 5 about z where y >= 0, thickened by 2
// toward its axis.
let m = model();
// The sheet is made from a solid: the half cylinder of radius 5 about z where y >= 0, from
// z = 0 to 10, with its three flat faces removed.
let p = m.profile(xy()).path(5.0, 0.0).arc_to([0.0, 5.0], [-5.0, 0.0]).line_to(5.0, 0.0).close()
    .sketch("half_disc").extrude(blind(10.0));
p = p.remove_faces(faces().kind("plane"), #{ label: "half_pipe" });
p = p.thicken(2.0, #{ side: "inward", label: "thick" });
m
