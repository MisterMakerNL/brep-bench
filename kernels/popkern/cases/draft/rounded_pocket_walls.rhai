// The block from the origin to (40, 30, 15) with a pocket from (8, 8, 5) to (32, 22, 15), its four
// vertical edges rounded to radius 4, cut into its top; the pocket's walls (planes and concave
// rounds, the faces centred at z = 10) drafted 5° about its floor's plane, z = 5, the pull +z.
let m = model();
let b = m.profile(xy()).rect(40.0, 30.0).at(20.0, 15.0).sketch("block").extrude(blind(15.0));
let pocket = m.tool(|m| {
    let p = m.profile(xy_at(5.0)).rect(24.0, 14.0).at(20.0, 15.0).sketch("pocket").extrude(blind(11.0));
    p.fillet(edges().direction("z"), 4.0, #{ label: "pocket_rounds" })
});
b = b.subtract(pocket, #{ label: "pocket" });
b = b.draft(faces().at_height(10.0), 5.0, xy_at(5.0), #{ label: "walls" });
m
