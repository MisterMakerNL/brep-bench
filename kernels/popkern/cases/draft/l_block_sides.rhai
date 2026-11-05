// The L-shaped block: the outline through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25), (0, 25),
// extruded from z = 0 to 12; all six sides drafted 6° about the floor's plane, z = 0, the pull +z.
let m = model();
let l = m.profile(xy())
    .polygon([[0.0, 0.0], [30.0, 0.0], [30.0, 10.0], [10.0, 10.0], [10.0, 25.0], [0.0, 25.0]])
    .sketch("outline").extrude(blind(12.0));
l = l.draft(faces().except(faces().facing("z").or(faces().facing_neg("z"))), 6.0, xy(), #{ label: "sides" });
m
