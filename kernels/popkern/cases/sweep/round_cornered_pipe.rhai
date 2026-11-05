// A pipe of radius 3 along two straight runs, (-30, 0, 0) to the origin and on along +y to
// (0, 30, 0), the section carried round the corner by turning it about the z axis.
// popkern's sweep follows one edge, not a path with a corner: each run is its disc swept straight
// along it, a cylinder ending flat at the corner, and the wedge they leave outside the corner,
// x >= 0, y <= 0, is the disc turned a quarter turn about the z axis: the quarter of the ball of
// radius 3 about the corner in that wedge: a sphere sector turning from -y to +x.
let m = model();
let first = m.profile(yz_at(-30.0)).circle(3.0).sketch("first_run").extrude(blind(30.0));
let second = m.tool(|m| m.profile(xz_at(-30.0)).circle(3.0).sketch("second_run")
    .extrude(blind(30.0)));
first = first.union(second, #{ label: "runs" });
let turn = m.tool(|m| m.sphere(3.0, #{ angle: 90.0, ref: [0.0, -1.0, 0.0], label: "quarter" }));
first = first.union(turn, #{ label: "corner" });
m
