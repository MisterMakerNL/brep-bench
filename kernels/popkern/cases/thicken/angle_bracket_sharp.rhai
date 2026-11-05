// A creased sheet, the floor (0, 0, 0)-(10, 8, 0) and the wall (0, 0, 0)-(0, 8, 10), thickened by 1
// outside the crease (to -z and -x), the corner kept sharp.
let m = model();
// The sheet is made from a solid: the box from the origin to (10, 8, 10), keeping only its
// floor (z = 0) and its wall x = 0, which share the crease along y.
let b = m.profile(xy()).rect(10.0, 8.0).at(5.0, 4.0).sketch("box").extrude(blind(10.0));
b = b.remove_faces(faces().except(faces().facing_neg("z").or(faces().facing_neg("x"))), #{ label: "crease" });
b = b.thicken(1.0, #{ side: "outward", join: "intersection", label: "thick" });
m
