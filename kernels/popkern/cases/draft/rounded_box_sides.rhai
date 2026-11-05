// The box from the origin to (30, 20, 10), its four vertical edges rounded to radius 4; all eight
// sides (four planes, four quarter cylinders) drafted 5° about the floor's plane, z = 0, the pull +z.
let m = model();
let b = m.profile(xy()).rect(30.0, 20.0).at(15.0, 10.0).sketch("box").extrude(blind(10.0));
b = b.fillet(edges().direction("z"), 4.0, #{ label: "rounds" });
b = b.draft(faces().except(faces().facing("z").or(faces().facing_neg("z"))), 5.0, xy(), #{ label: "sides" });
m
