// A cylinder of radius 5 on the z axis from z = 0 to 10, shelled inward to a wall 1 thick, open at
// its curved side: only its two ends are left, as two discs 1 thick.
let m = model();
let c = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
c = c.shell(faces().kind("cylinder"), 1.0, #{ label: "hollow" });
m
