// The box from the origin to (20, 12, 10), its four sides drafted 5° about the floor's plane, z = 0,
// the pull +z: each leans in as it rises.
let m = model();
let b = m.profile(xy()).rect(20.0, 12.0).at(10.0, 6.0).sketch("box").extrude(blind(10.0));
b = b.draft(faces().except(faces().facing("z").or(faces().facing_neg("z"))), 5.0, xy(), #{ label: "sides" });
m
