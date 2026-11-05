// The plate from (-10, -10, 0) to (10, 10, 4), drilled through along the z axis by a hole of
// radius 3, grown by 1.
let m = model();
let plate = m.profile(xy()).rect(20.0, 20.0).sketch("plate").extrude(blind(4.0));
let drill = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).sketch("drill").extrude(blind(6.0)));
plate = plate.subtract(drill, #{ label: "hole" });
plate = plate.offset_solid(1.0, #{ label: "grown" });
m
