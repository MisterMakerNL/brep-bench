// The box from the origin to (10, 8, 6), shelled inward to a wall 4 thick, open at its top, z = 6:
// the walls at y = 0 and y = 8, moved 4 inward, meet in the plane y = 4.
let m = model();
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(6.0));
b = b.shell(faces().facing("z").at_height(6.0), 4.0, #{ label: "hollow" });
m
