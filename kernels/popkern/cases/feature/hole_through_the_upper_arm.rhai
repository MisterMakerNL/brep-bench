// A C-shaped bracket: the polygon through (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24),
// (40, 24), (40, 30), (0, 30) at y = -10, extruded to y = 10. A hole of radius 4 on the vertical
// axis through (28, 0), drilled down from the upper arm's top until it comes out into the gap:
// a cut sketched facing down on the top's plane, run up to the arm's underside at z = 24.
// plane_at's frame for the normal -z has u along -y and v along -x, so x = 28 is v = -28.
let m = model();
let bracket = m.profile(xz_at(-10.0))
    .polygon([[0.0, 0.0], [40.0, 0.0], [40.0, 6.0], [6.0, 6.0], [6.0, 24.0], [40.0, 24.0],
        [40.0, 30.0], [0.0, 30.0]])
    .sketch("bracket").extrude(blind(20.0));
let hole = m.profile(plane_at([0.0, 0.0, 30.0], [0.0, 0.0, -1.0])).circle(4.0).at(0.0, -28.0)
    .sketch("hole")
    .extrude(up_to_surface(bracket, faces().facing_neg("z").at_height(24.0)), #{ mode: "cut", label: "drill" });
m
