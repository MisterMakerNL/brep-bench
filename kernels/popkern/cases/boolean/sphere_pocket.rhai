fn ball(m, r) {
    m.sphere(r)
}

// A block from (-10, -10, -10) to (10, 10, 0), with a ball of radius 5 on the origin cut from it.
let m = model();
let block = m.profile(xy_at(-10.0)).rect(20.0, 20.0).sketch("block").extrude(blind(10.0));
let pocket = m.tool(|m| ball(m, 5.0));
block = block.subtract(pocket);
m
