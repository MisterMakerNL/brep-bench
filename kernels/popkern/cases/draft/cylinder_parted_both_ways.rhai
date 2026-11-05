// A cylinder of radius 10 on the z axis from z = 0 to 20, parted at z = 10 and each half's side
// drafted 3° away from the parting line: the upper half with the pull +z, the lower with -z, both
// about the plane z = 10. Each half is drafted as its own cylinder, then the two are united.
let m = model();
let upper = m.profile(xy_at(10.0)).circle(10.0).sketch("upper").extrude(blind(10.0));
upper = upper.draft(faces().kind("cylinder"), 3.0, xy_at(10.0), #{ label: "upper_side" });
let lower = m.tool(|m| {
    let l = m.profile(xy()).circle(10.0).sketch("lower").extrude(blind(10.0));
    l.draft(faces().kind("cylinder"), 3.0, plane_at([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]),
        #{ label: "lower_side" })
});
upper = upper.union(lower, #{ label: "parted" });
m
