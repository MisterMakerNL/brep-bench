// The box from the origin to (10, 8, 6), hollowed to a wall 1 thick with no opening: no face is
// removed, so the shell leaves a closed cavity.
let m = model();
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(6.0));
b = b.shell(faces().kind("sphere").allow_empty(), 1.0, #{ label: "hollow" });
m
