// The closed uniform periodic cubic B-spline over the peanut's twelve control points, extruded
// from z = 0 to z = 10.
let m = model();
let p = m.profile(xy()).spline([[14.0, 0.0], [12.0, 8.0], [4.0, 6.0], [0.0, -2.625], [-4.0, 6.0],
    [-12.0, 8.0], [-14.0, 0.0], [-12.0, -8.0], [-4.0, -6.0], [0.0, 2.625], [4.0, -6.0],
    [12.0, -8.0]], #{ degree: 3 }).sketch("peanut").extrude(blind(10.0));
m
