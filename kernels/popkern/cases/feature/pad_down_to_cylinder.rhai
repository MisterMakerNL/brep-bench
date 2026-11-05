// A cylinder of radius 10 along the x axis from x = -20 to 20, and a square pad, |x| <= 3,
// |y| <= 3 in the plane z = 15, extruded down until it meets the cylinder's side, joined to it.
let m = model();
let drum = m.profile(yz_at(-20.0)).circle(10.0).sketch("cylinder").extrude(blind(40.0));
let pad = m.profile(plane_at([0.0, 0.0, 15.0], [0.0, 0.0, -1.0])).rect(6.0, 6.0).sketch("pad")
    .extrude(up_to_surface(drum, faces().kind("cylinder")), #{ mode: "boss", label: "pad" });
m
