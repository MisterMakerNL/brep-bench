// A disc of radius 20 swept along the planar cubic Bézier (0, 0), (12, −16), (32, −16), (44, 0).
// The rail is a slab on the xy plane whose outline is the cubic closed by its chord; the pipe
// follows its NURBS edge at z = 0.
let m = model();
let rail = m.profile(xy()).path(0.0, 0.0)
    .spline_to([[12.0, -16.0], [32.0, -16.0], [44.0, 0.0]], #{ degree: 3 }).close().sketch("rail").extrude(blind(1.0));
rail.pipe(edges().nurbs().at_height(0.0), 20.0, #{ label: "pipe" });
m
