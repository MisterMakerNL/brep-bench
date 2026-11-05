// A closed hollow dome: a plate, the disc of radius 20 from z = -2 to 0, and on it everything above
// z = 0 between 18 and 20 from the origin. A post of radius 3 on the z axis rises from the plate's
// top until it meets the inside of the dome, the sphere of radius 18, and is joined to the dome.
// popkern has no ball primitive: each half ball is a quarter disc revolved about the z axis.
fn half_ball(m, r) {
    m.profile(xz()).path(0.0, 0.0).line_to(r, 0.0).arc_to([r * 0.6, r * 0.8], [0.0, r]).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}
let m = model();
let dome = half_ball(m, 20.0);
let plate = m.tool(|m| m.profile(xy_at(-2.0)).circle(20.0).sketch("plate").extrude(blind(2.0)));
dome = dome.union(plate, #{ label: "plate" });
let cavity = m.tool(|m| half_ball(m, 18.0));
m.set_active(cavity);
m.label_face("inside", faces().kind("sphere"));
dome = dome.subtract(cavity, #{ label: "cavity" });
let post = m.profile(xy()).circle(3.0).sketch("post")
    .extrude(up_to_surface(dome, faces().labeled("inside")), #{ mode: "boss", label: "post" });
m
