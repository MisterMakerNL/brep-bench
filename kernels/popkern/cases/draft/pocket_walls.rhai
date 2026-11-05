// The block from the origin to (40, 30, 15) with a pocket from (8, 8, 5) to (32, 22, 15) cut into its
// top; the pocket's four walls (the faces centred at z = 10) drafted 5° about its floor's plane,
// z = 5, the pull +z: the pocket widens toward its opening.
let m = model();
let b = m.profile(xy()).rect(40.0, 30.0).at(20.0, 15.0).sketch("block").extrude(blind(15.0));
let pocket = m.tool(|m| m.profile(xy_at(5.0)).rect(24.0, 14.0).at(20.0, 15.0).sketch("pocket")
    .extrude(blind(11.0)));
b = b.subtract(pocket, #{ label: "pocket" });
b = b.draft(faces().at_height(10.0), 5.0, xy_at(5.0), #{ label: "walls" });
m
