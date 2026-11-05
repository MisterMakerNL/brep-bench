fn ball(m, r) {
    m.sphere(r)
}

// Two balls of radius 5, on the origin and on (6, 0, 0), united.
let m = model();
let pair = ball(m, 5.0);
let other = m.tool(|m| ball(m, 5.0).translate(6.0, 0.0, 0.0));
pair = pair.union(other);
m
