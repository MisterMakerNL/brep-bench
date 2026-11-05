// A cube of side 10 from the origin, its twelve edges chamfered 2 on both faces, each chamfer
// running on until it meets the others: the three at each corner meet in a point.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
cube = cube.chamfer(edges(), 2.0, #{ label: "bevels" });
m
