// An L-shaped plate extruded 8 along +z, shelled inward to a wall 1 thick, open at its top, z = 8:
// the wall is every point within 1 of the other faces, so at the concave edge x = 5, y = 5 its
// inside is rounded (the arc join).
let m = model();
let p = m.profile(xy()).polygon([[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0],
    [0.0, 15.0]]).sketch("l_outline").extrude(blind(8.0));
p = p.shell(faces().facing("z").at_height(8.0), 1.0, #{ join: "arc", label: "hollow" });
m
