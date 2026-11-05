// The box from the origin to (10, 8, 6), shelled inward to a wall 1 thick, open at its top, z = 6,
// and its front, y = 0: two openings that share an edge.
let m = model();
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(6.0));
b = b.shell(faces().facing("z").or(faces().facing_neg("y")), 1.0, #{ label: "hollow" });
m
