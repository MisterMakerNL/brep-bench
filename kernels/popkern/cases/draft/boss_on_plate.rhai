// The plate from (-20, -20, 0) to (20, 20, 5) with a cylindrical boss of radius 8 on the z axis from
// z = 5 to 17 united onto it; the boss's side drafted 4° about the plate's top, z = 5, the pull +z.
let m = model();
let plate = m.profile(xy()).rect(40.0, 40.0).sketch("plate").extrude(blind(5.0));
let boss = m.tool(|m| m.profile(xy_at(5.0)).circle(8.0).sketch("boss").extrude(blind(12.0)));
plate = plate.union(boss, #{ label: "boss" });
plate = plate.draft(faces().kind("cylinder"), 4.0, xy_at(5.0), #{ label: "boss_side" });
m
