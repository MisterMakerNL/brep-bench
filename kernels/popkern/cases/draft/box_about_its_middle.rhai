// The box from the origin to (20, 12, 10), its four sides drafted 5° about the plane z = 5, halfway
// up, the pull +z.
let m = model();
let b = m.profile(xy()).rect(20.0, 12.0).at(10.0, 6.0).sketch("box").extrude(blind(10.0));
b = b.draft(faces().except(faces().facing("z").or(faces().facing_neg("z"))), 5.0, xy_at(5.0), #{ label: "sides" });
m
