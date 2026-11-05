// The cylinder of radius 5 about z from z = 0 to 10 as a sheet, thickened by 1 away from its axis.
// popkern scripts make sheets from solids: the cylinder with its two ends removed.
let m = model();
let c = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
c = c.remove_faces(faces().kind("plane"), #{ label: "side" });
c = c.thicken(1.0, #{ side: "outward", label: "thick" });
m
