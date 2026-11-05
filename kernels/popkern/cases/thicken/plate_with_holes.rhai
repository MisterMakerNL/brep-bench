// A square sheet (-10, -10)-(10, 10) with two holes of radius 3 at (-5, 0) and (5, 0), thickened by
// 1 to each side. The sheet is made from a solid: a plate 1 thick drilled twice, keeping
// only its top face.
let m = model();
let p = m.profile(xy()).rect(20.0, 20.0).sketch("plate").extrude(blind(1.0));
for x in [-5.0, 5.0] {
    let drill = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).at(x, 0.0).sketch("drill")
        .extrude(blind(3.0)));
    p = p.subtract(drill);
}
p = p.remove_faces(faces().except(faces().facing("z")), #{ label: "sheet" });
p = p.thicken(2.0, #{ side: "both", label: "thick" });
m
