// A picture frame: a moulding swept round the rectangle with corners (±30, ±20, 0), mitred at its
// four corners by the vertical planes along their diagonals. The section, at distance s outward
// from the path and height z, is the polygon (s, z) = (0, 2), (3, 2), (3, 0), (8, 0), (8, 6), (0, 6).
// popkern's sweep follows one edge, not a closed path with corners: each side is the section
// extruded straight along it, past the corners, and cut back to its side of the two mitre planes.
// The sides y = 20 and x = -30 are those at y = -20 and x = 30 turned a half turn about the z axis.
let m = model();
// everything on the side of the plane through `o` that the normal `n` points to
fn beyond(m, o, n, label) {
    m.tool(|m| m.profile(plane_at(o, n)).rect(300.0, 300.0).sketch(label).extrude(blind(150.0)))
}
// the side y = -20, from x = -30 to 30: the section in the plane x = -40 (u = y, v = z)
fn long_side(m) {
    let k = 1.0 / sqrt(2.0);
    let side = m.profile(yz_at(-40.0))
        .polygon([[-20.0, 2.0], [-23.0, 2.0], [-23.0, 0.0], [-28.0, 0.0], [-28.0, 6.0], [-20.0, 6.0]])
        .sketch("long_section").extrude(blind(80.0));
    side = side.subtract(beyond(m, [30.0, -20.0, 0.0], [k, k, 0.0], "long_right_mitre"));
    side.subtract(beyond(m, [-30.0, -20.0, 0.0], [-k, k, 0.0], "long_left_mitre"))
}
// the side x = 30, from y = -20 to 20: the section in the plane y = 30 (u = x, v = z)
fn short_side(m) {
    let k = 1.0 / sqrt(2.0);
    let side = m.profile(xz_at(-30.0))
        .polygon([[30.0, 2.0], [33.0, 2.0], [33.0, 0.0], [38.0, 0.0], [38.0, 6.0], [30.0, 6.0]])
        .sketch("short_section").extrude(blind(60.0));
    side = side.subtract(beyond(m, [30.0, -20.0, 0.0], [-k, -k, 0.0], "short_lower_mitre"));
    side.subtract(beyond(m, [30.0, 20.0, 0.0], [-k, k, 0.0], "short_upper_mitre"))
}
let z_axis = axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
let frame = long_side(m);
frame = frame.union(m.tool(|m| short_side(m)), #{ label: "corner_1" });
frame = frame.union(m.tool(|m| long_side(m).rotate(z_axis, 180.0)), #{ label: "corner_2" });
frame = frame.union(m.tool(|m| short_side(m).rotate(z_axis, 180.0)), #{ label: "corner_3" });
m
