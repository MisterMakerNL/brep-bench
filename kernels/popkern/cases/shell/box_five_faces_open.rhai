// The box from the origin to (10, 8, 6), shelled inward to a wall 1 thick, open at every face but
// its bottom, z = 0: what is left is the floor.
let m = model();
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(6.0));
b = b.shell(faces().except(faces().facing_neg("z")), 1.0, #{ label: "hollow" });
m
