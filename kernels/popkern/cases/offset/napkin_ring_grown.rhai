// A napkin ring: a ball of radius 5 on the origin (a half disc revolved about z) bored through
// along z by a cylinder of radius 3, grown by 1.
let m = model();
let ring = m.profile(xz()).path(0.0, -5.0).arc_to([5.0, 0.0], [0.0, 5.0]).line_to(0.0, -5.0).close()
    .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
let bore = m.tool(|m| m.profile(xy_at(-6.0)).circle(3.0).sketch("bore").extrude(blind(12.0)));
ring = ring.subtract(bore, #{ label: "bore" });
ring = ring.offset_solid(1.0, #{ label: "grown" });
m
