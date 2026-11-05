// The bent flange thickened by 3 inside its bend, past the bend's radius of 2.
let m = model();
// popkern scripts make sheets from solids: the region bounded by the flange (the line from
// (-10, 0) to (0, 0), the quarter circle of radius 2 about (0, 2) to (2, 2), the line up to (2, 12))
// and closed above, in the xz plane, extruded 8; then every face but the flange's three removed.
let f = m.profile(xz()).path(-10.0, 0.0).line_to(0.0, 0.0)
    .arc_to([1.4142135623730951, 0.5857864376269049], [2.0, 2.0]).line_to(2.0, 12.0)
    .line_to(-10.0, 12.0).close()
    .sketch("flange_region").extrude(blind(8.0));
f = f.remove_faces(faces().facing("z").or(faces().facing_neg("x")).or(faces().facing("y"))
    .or(faces().facing_neg("y")), #{ label: "flange" });
// The sheet keeps the region's outward normals: inside the bend (above the floor, before the wall)
// is inward.
f = f.thicken(3.0, #{ side: "inward", label: "thick" });
m
