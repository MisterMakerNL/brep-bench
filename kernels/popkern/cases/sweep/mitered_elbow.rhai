// A square bar 10 x 10, its sides horizontal at z = -5 and 5, swept along two straight runs, from
// the origin along +x to (40, 0, 0), then turning 120° to the left on 40 to (20, 20√3, 0), mitred
// at the corner by the plane x + √3 y = 40 that bisects it: a sweep along the two-edge wire.
let m = model();
let path = m.wire_polyline([[0.0, 0.0, 0.0], [40.0, 0.0, 0.0], [20.0, 20.0 * sqrt(3.0), 0.0]],
    #{ label: "path" });
let bar = m.profile(yz()).rect(10.0, 10.0).sketch("section").sweep(path, #{ label: "elbow" });
m
