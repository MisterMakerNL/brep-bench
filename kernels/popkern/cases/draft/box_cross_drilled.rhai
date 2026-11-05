// The box from the origin to (30, 20, 20), drilled through along x by a hole of radius 4 on the line
// y = 10, z = 10; its two drilled faces, x = 0 and x = 30, drafted 8° about the floor's plane,
// z = 0, the pull +z.
let m = model();
let b = m.profile(xy()).rect(30.0, 20.0).at(15.0, 10.0).sketch("box").extrude(blind(20.0));
let drill = m.tool(|m| m.profile(yz_at(-1.0)).circle(4.0).at(10.0, 10.0).sketch("drill")
    .extrude(blind(32.0)));
b = b.subtract(drill, #{ label: "hole" });
b = b.draft(faces().facing("x").or(faces().facing_neg("x")), 8.0, xy(), #{ label: "ends" });
m
