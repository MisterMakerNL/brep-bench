// A pipe of radius 2 along three edges of a cube of side 30, (0, 0, 0) to (30, 0, 0) to
// (30, 30, 0) to (30, 30, 30), mitred at its two corners by the planes x + y = 30 and y + z = 30.
// popkern's sweep along the three-edge wire serves each mitred run but does not unite them on
// their elliptical mitre faces: each run is the disc extruded straight along it, past its
// corners, and cut back to its side of each mitre plane; the three are then united.
fn beyond(m, at, normal) {
    m.tool(|m| m.profile(plane_at(at, normal)).rect(200.0, 200.0).sketch("beyond").extrude(blind(100.0)))
}
let m = model();
let s = 1.0 / sqrt(2.0);
let first = m.cylinder(2.0, 40.0, #{ axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0], label: "first" });
first = first.subtract(beyond(m, [30.0, 0.0, 0.0], [s, s, 0.0]));
let second = m.tool(|m| {
    let r = m.cylinder(2.0, 50.0, #{ at: [30.0, -10.0, 0.0], axis: [0.0, 1.0, 0.0], ref: [0.0, 0.0, 1.0] });
    r = r.subtract(beyond(m, [30.0, 0.0, 0.0], [-s, -s, 0.0]));
    r.subtract(beyond(m, [30.0, 30.0, 0.0], [0.0, s, s]))
});
let third = m.tool(|m| {
    let r = m.cylinder(2.0, 40.0, #{ at: [30.0, 30.0, -10.0] });
    r.subtract(beyond(m, [30.0, 30.0, 0.0], [0.0, -s, -s]))
});
first = first.union(second);
first = first.union(third, #{ label: "pipe" });
m
