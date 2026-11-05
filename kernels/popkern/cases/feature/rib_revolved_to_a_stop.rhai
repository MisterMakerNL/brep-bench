// A hub, the cylinder of radius 10 on the z axis from z = 0 to 10; a stop, everything within 25 of
// the axis between the half-planes at 120° and 135°, z = 0 to 10; and a rib, the rectangle
// 5 <= x <= 20, 0 <= z <= 4 in the plane y = 0, turned from 0° towards +y until it meets the stop's
// face at 120°. All three joined.
// The stop is drawn between 0° and 15°, its face on the x axis labelled, and turned 120° into
// place; the rib is turned up to that face as a boss into the stop.
let m = model();
let z = axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
let hub = m.profile(xy()).circle(10.0).sketch("hub").extrude(blind(10.0));
let stop = m.profile(xy()).path(0.0, 0.0).line_to(25.0, 0.0)
    .arc_to([25.0 * cos(PI() / 24.0), 25.0 * sin(PI() / 24.0)],
        [25.0 * cos(PI() / 12.0), 25.0 * sin(PI() / 12.0)])
    .close().sketch("stop").extrude(blind(10.0));
m.label_face("stop_face", faces().facing_neg("y"));
stop = stop.rotate(z, 120.0);
let rib = m.profile(xz()).rect(15.0, 4.0).at(12.5, 2.0).sketch("rib")
    .revolve(z, up_to_surface(stop, faces().labeled("stop_face")), #{ mode: "boss", label: "rib" });
hub = hub.union(stop, #{ label: "stop" });
m
