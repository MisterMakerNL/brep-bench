// A block with a groove cut by a ball of radius 3 whose centre follows a cubic Bézier curve in
// the top face: the tube of radius 3 about the curve (a pipe along the Bézier wire), a ball at
// each end, cut from the block.
let m = model();
let block = m.box(42.0, 32.0, 8.0, #{ at: [-6.0, -6.0, -8.0], label: "block" });
let path = m.wire_spline([[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [20.0, 20.0, 0.0], [30.0, 20.0, 0.0]],
    #{ control: true, label: "path" });
let groove = m.tool(|m| {
    let tube = path.pipe(3.0, #{ label: "tube" });
    let a = m.tool(|m| m.sphere(3.0, #{ label: "start" }));
    let b = m.tool(|m| m.sphere(3.0, #{ at: [30.0, 20.0, 0.0], label: "end" }));
    tube = tube.union(a);
    tube.union(b)
});
block = block.subtract(groove, #{ label: "groove" });
m
