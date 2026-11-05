fn ball(m, r) {
    m.sphere(r)
}

// A ball of radius 5 on the origin, bored through along z by a cylinder of radius 3.
let m = model();
let ring = ball(m, 5.0);
let bore = m.tool(|m| m.profile(xy_at(-6.0)).circle(3.0).sketch("bore").extrude(blind(12.0)));
ring = ring.subtract(bore);
m
