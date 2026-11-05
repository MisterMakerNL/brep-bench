// The box from the origin to (10, 8, 6), shelled inward to a wall 0.001 thick, open at its top,
// z = 6.
let m = model();
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(6.0));
b = b.shell(faces().facing("z").at_height(6.0), 0.001, #{ label: "hollow" });
m
