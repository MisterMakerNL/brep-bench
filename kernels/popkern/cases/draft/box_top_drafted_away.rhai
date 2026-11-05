// The box from the origin to (30, 10, 10), its four sides drafted 30° about the floor's plane,
// z = 0, the pull +z: the long sides meet below the top, which the draft consumes.
let m = model();
let b = m.profile(xy()).rect(30.0, 10.0).at(15.0, 5.0).sketch("box").extrude(blind(10.0));
b = b.draft(faces().except(faces().facing("z").or(faces().facing_neg("z"))), 30.0, xy(), #{ label: "sides" });
m
