// The periodic uniform quadratic B-spline over the square (10, 10), (−10, 10), (−10, −10),
// (10, −10), extruded from z = 0 to z = 10.
let m = model();
let p = m.profile(xy()).spline([[10.0, 10.0], [-10.0, 10.0], [-10.0, -10.0], [10.0, -10.0]],
    #{ degree: 2 }).sketch("section").extrude(blind(10.0));
m
