// The block from (-15, -15, 0) to (15, 15, 20), bored through along z by a hole of radius 6; the
// hole's wall drafted 3° about the floor's plane, z = 0, the pull +z: the hole widens upward.
let m = model();
let b = m.profile(xy()).rect(30.0, 30.0).sketch("block").extrude(blind(20.0));
let bore = m.tool(|m| m.profile(xy_at(-1.0)).circle(6.0).sketch("bore").extrude(blind(22.0)));
b = b.subtract(bore, #{ label: "hole" });
b = b.draft(faces().kind("cylinder"), 3.0, xy(), #{ label: "taper" });
m
