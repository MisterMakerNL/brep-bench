// A cylinder of radius 5 on the z axis from z = 0 to 10, shelled inward to a wall 1 thick, open at
// its top, z = 10.
let m = model();
let c = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
c = c.shell(faces().kind("plane").at_height(10.0), 1.0, #{ label: "hollow" });
m
