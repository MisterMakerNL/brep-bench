// Two 10 x 10 squares joined by a neck 4 long and 2 wide, extruded 6 along +z, shelled inward to a
// wall 1.5 thick, open at its top, z = 6: the neck, narrower than two walls, is left solid.
let m = model();
let p = m.profile(xy()).polygon([[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [14.0, 4.0], [14.0, 0.0],
    [24.0, 0.0], [24.0, 10.0], [14.0, 10.0], [14.0, 6.0], [10.0, 6.0], [10.0, 10.0], [0.0, 10.0]])
    .sketch("dumbbell").extrude(blind(6.0));
p = p.shell(faces().facing("z").at_height(6.0), 1.5, #{ label: "hollow" });
m
