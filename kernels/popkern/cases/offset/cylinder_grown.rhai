// A cylinder of radius 5 on the z axis from z = 0 to 10, grown by 1.
let m = model();
let c = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
c = c.offset_solid(1.0, #{ label: "grown" });
m
