// A transition piece: a loft from the square -20 <= x, y <= 20 at z = 0 to the octagon at z = 30
// with corners (6, -14), (14, -6), (14, 6), (6, 14), (-6, 14), (-14, 6), (-14, -6), (-6, -14),
// each side of the square joined to the octagon's side parallel to it, each corner to the cut side
// facing it.
let m = model();
let square = m.profile(xy()).rect(40.0, 40.0).sketch("square");
let octagon = m.profile(xy_at(30.0))
    .polygon([[6.0, -14.0], [14.0, -6.0], [14.0, 6.0], [6.0, 14.0], [-6.0, 14.0], [-14.0, 6.0],
        [-14.0, -6.0], [-6.0, -14.0]])
    .sketch("octagon");
m.loft([square, octagon], #{ label: "hood" });
m
