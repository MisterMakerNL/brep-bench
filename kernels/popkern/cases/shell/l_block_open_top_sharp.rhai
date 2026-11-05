// An L-shaped plate extruded 8 along +z, shelled inward to a wall 1 thick, open at its top, z = 8,
// every pair of moved faces extended until they meet, the concave edge's included (the
// intersection join).
let m = model();
let p = m.profile(xy()).polygon([[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0],
    [0.0, 15.0]]).sketch("l_outline").extrude(blind(8.0));
p = p.shell(faces().facing("z").at_height(8.0), 1.0, #{ join: "intersection", label: "hollow" });
m
