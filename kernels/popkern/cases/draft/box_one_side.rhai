// The box from the origin to (20, 12, 10), its side x = 20 drafted 10° about the floor's plane,
// z = 0, the pull +z.
let m = model();
let b = m.profile(xy()).rect(20.0, 12.0).at(10.0, 6.0).sketch("box").extrude(blind(10.0));
b = b.draft(faces().facing("x"), 10.0, xy(), #{ label: "side" });
m
