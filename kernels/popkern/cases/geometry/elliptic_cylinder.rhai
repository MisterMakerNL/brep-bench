// The ellipse x²/25 + y²/9 ≤ 1 on the xy plane, extruded from z = 0 to z = 10.
let m = model();
let c = m.profile(xy()).ellipse(5.0, 3.0).sketch("section").extrude(blind(10.0));
m
