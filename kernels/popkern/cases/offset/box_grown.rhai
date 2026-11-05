// The box from the origin to (10, 8, 6), grown by 1: every face moved out 1, its edges rounded to
// quarter cylinders and its corners to eighth balls (the default arc join).
let m = model();
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(6.0));
b = b.offset_solid(1.0, #{ label: "grown" });
m
