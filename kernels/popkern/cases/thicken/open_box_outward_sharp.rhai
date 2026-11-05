// An open box's sheet thickened by 1 out of the cube, its edges kept sharp.
let m = model();
// popkern scripts make sheets from solids: the cube from the origin to (10, 10, 10) with its top
// removed, an open tray.
let b = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
b = b.remove_faces(faces().facing("z"), #{ label: "tray" });
b = b.thicken(1.0, #{ side: "outward", join: "intersection", label: "thick" });
m
