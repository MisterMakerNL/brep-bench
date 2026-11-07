+++
title = "A box with its top edges chamfered unequally, twenty ways"
tags = ["fuzz", "chamfer", "asymmetric", "corner"]

[params]
a = [5, 30]
b = [5, 30]
c = [3, 20]
d1 = ["0.02*min(a, b)", "0.45*min(a, b)"]
d2 = ["0.02*c", "0.95*c"]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[volume]
formula = "a*b*(c - d2) + d2*(a*b - d1*(a + b) + 4*d1**2/3)"
exact = "ab(c − d₂) + ∫₀^d₂ (a − 2d₁t/d₂)(b − 2d₁t/d₂) dt = ab(c − d₂) + d₂(ab − d₁(a + b) + 4d₁²/3)"
tex = 'ab(c - d_2) + d_2\left(ab - d_1(a + b) + \tfrac{4}{3}d_1^2\right)'

[area]
formula = "a*b + (a - 2*d1)*(b - 2*d1) + 2*(a + b)*(c - d2) + 2*(a + b - 2*d1)*sqrt(d1**2 + d2**2)"
exact = "ab + (a − 2d₁)(b − 2d₁) + 2(a + b)(c − d₂) + 2(a + b − 2d₁)√(d₁² + d₂²)"
tex = 'ab + (a - 2d_1)(b - 2d_1) + 2(a + b)(c - d_2) + 2(a + b - 2d_1)\sqrt{d_1^2 + d_2^2}'
+++

A box from the origin to (`a`, `b`, `c`). Chamfer the four edges around its top face z = `c`, each
with distance `d1` measured across the top face and distance `d2` measured down the side face: the
chamfer of the edge at y = `b`, z = `c` is the plane through the lines y = `b` − `d1`, z = `c` and
y = `b`, z = `c` − `d2`, and the other three likewise.

The solid is the box cut by those four planes. Its top face is the rectangle from (`d1`, `d1`, `c`)
to (`a` − `d1`, `b` − `d1`, `c`); its four vertical edges end at z = `c` − `d2`, and the chamfers of
two neighboring edges meet along a slanting line from the box's corner at that height to the top
face's corner. Each chamfer face is a trapezoid; four faces meet at each of the points such as
(`a`, `b`, `c` − `d2`).

Then turn it by `psi` about the axis through the origin with direction (sin `theta` cos `phi`,
sin `theta` sin `phi`, cos `theta`), right-handed.

`chamfer/box-top-asymmetric`, of any proportions, its chamfers from a hair to nearly the whole
side.
