// The octahedron |x| + |y| + |z| <= 10, carved from a cube by its eight face planes, its twelve
// edges chamfered √6 on both faces; four chamfers meet at each corner, which is cut flat.
let m = model();
let solid = m.profile(xy_at(-10.0)).rect(20.0, 20.0).sketch("blank").extrude(blind(20.0));
let r = 10.0 / sqrt(3.0);
let deep = 40.0;
let i = 0;
for sx in [1.0, -1.0] { for sy in [1.0, -1.0] { for sz in [1.0, -1.0] {
    let k = 1.0 / sqrt(3.0);
    let n = [sx * k, sy * k, sz * k];
    let o = [(r - deep) * n[0], (r - deep) * n[1], (r - deep) * n[2]];
    let slab = m.tool(|t| t.profile(plane_at(o, n)).rect(80.0, 80.0).sketch("face_" + i).extrude(blind(deep)));
    solid = solid.intersect(slab, #{ label: "cut_" + i });
    i += 1;
} } }
solid = solid.chamfer(edges(), sqrt(6.0), #{ label: "bevels" });
m
