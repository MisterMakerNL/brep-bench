// A square-to-round: a loft from the square -20 <= x, y <= 20 at z = 0 to the circle of radius 15
// about the z axis at z = 30.
let m = model();
let square = m.profile(xy()).rect(40.0, 40.0).sketch("square");
let round = m.profile(xy_at(30.0)).circle(15.0).sketch("round");
m.loft([square, round], #{ label: "transition" });
m
