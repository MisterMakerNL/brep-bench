// A 20 × 20 square swept along the planar cubic Bézier (0, 0), (16, 12), (18, 26), (12, 34). The
// rail is a sketch of the xy plane whose outline is the cubic closed by its chord; the sweep
// follows its NURBS edge at z = 0. The section starts square to the curve at the origin, along (4, 3, 0).
let m = model();
let rail = m.profile(xy()).path(0.0, 0.0)
    .spline_to([[16.0, 12.0], [18.0, 26.0], [12.0, 34.0]], #{ degree: 3 }).close().sketch("rail").extrude(blind(1.0));
let bar = m.profile(plane_at([0.0, 0.0, 0.0], [0.8, 0.6, 0.0])).rect(20.0, 20.0).sketch("section");
bar = bar.sweep(rail, edges().nurbs().at_height(0.0), #{ label: "bar" });
m
