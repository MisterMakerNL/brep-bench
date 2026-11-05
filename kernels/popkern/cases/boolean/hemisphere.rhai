fn ball(m, r) {
    m.sphere(r)
}

// A ball of radius 5 on the origin, with everything below the XY plane cut away by a block.
let m = model();
let dome = ball(m, 5.0);
let below = m.tool(|m| m.profile(xy_at(-6.0)).rect(12.0, 12.0).sketch("below").extrude(blind(6.0)));
dome = dome.subtract(below);
m
