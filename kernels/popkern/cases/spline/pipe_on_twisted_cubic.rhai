// A pipe of radius 8 swept along a twisted (non-planar) cubic Bézier curve: the Bézier is a wire
// spline over its four control points, and the pipe follows it.
let m = model();
let path = m.wire_spline([[0.0, 0.0, 0.0], [12.0, 16.0, 0.0], [26.0, 8.0, 0.0], [24.0, 0.0, 16.0]],
    #{ control: true, label: "path" });
let pipe = path.pipe(8.0, #{ label: "pipe" });
m
