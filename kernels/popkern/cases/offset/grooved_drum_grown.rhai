// A drum, the cylinder of radius 10 on the z axis from z = 0 to 10, with a groove round its side
// from z = 4.2 to 5.8 cut to radius 7; grown by 1, which fills the groove.
let m = model();
let drum = m.profile(xy()).circle(10.0).sketch("drum").extrude(blind(10.0));
let groove = m.tool(|m| m.profile(xz())
    .path(7.0, 4.2).line_to(11.0, 4.2).line_to(11.0, 5.8).line_to(7.0, 5.8).close()
    .sketch("groove_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()));
drum = drum.subtract(groove, #{ label: "groove" });
drum = drum.offset_solid(1.0, #{ label: "grown" });
m
