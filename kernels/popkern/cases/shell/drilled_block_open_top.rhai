// The block from the origin to (20, 20, 10), bored through along z by a hole of radius 3 about
// x = 10, y = 10, then shelled inward to a wall 1 thick, open at its top, z = 10.
let m = model();
let b = m.profile(xy()).rect(20.0, 20.0).at(10.0, 10.0).sketch("block").extrude(blind(10.0));
let bore = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).at(10.0, 10.0).sketch("bore")
    .extrude(blind(12.0)));
b = b.subtract(bore);
b = b.shell(faces().facing("z").at_height(10.0), 1.0, #{ label: "hollow" });
m
