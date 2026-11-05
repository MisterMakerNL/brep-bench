// A drum, the cylinder of radius 8 on the z axis from z = 0 to 6, with a flange round its side
// from radius 8 out to 11 and from z = 2.2 to 3.8; shrunk by 1, which takes the flange away.
let m = model();
let drum = m.profile(xy()).circle(8.0).sketch("drum").extrude(blind(6.0));
let flange = m.tool(|m| m.profile(xy_at(2.2)).circle(11.0).sketch("flange").extrude(blind(1.6)));
drum = drum.union(flange, #{ label: "flanged" });
drum = drum.offset_solid(-1.0, #{ label: "shrunk" });
m
