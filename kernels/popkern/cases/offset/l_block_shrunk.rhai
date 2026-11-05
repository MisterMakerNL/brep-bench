// The L-shaped plate: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15),
// extruded 8 up, shrunk by 1.
let m = model();
let l = m.profile(xy())
    .polygon([[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]])
    .sketch("outline").extrude(blind(8.0));
l = l.offset_solid(-1.0, #{ label: "shrunk" });
m
