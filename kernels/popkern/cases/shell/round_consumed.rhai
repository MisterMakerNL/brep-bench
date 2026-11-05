// The box from the origin to (20, 14, 10), its four vertical edges rounded to radius 2, shelled
// inward to a wall 3 thick, open at its top, z = 10.
let m = model();
let p = m.profile(xy()).rect(20.0, 14.0).at(10.0, 7.0).sketch("plate").extrude(blind(10.0));
p = p.fillet(edges().straight().direction("z"), 2.0, #{ label: "corners" });
p = p.shell(faces().facing("z").at_height(10.0), 3.0, #{ label: "hollow" });
m
