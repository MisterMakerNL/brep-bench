+++
title = "A square bar mitered around a sharp corner"
tags = ["sweep", "planar-path", "corner", "miter", "analytic"]

[volume]
value = 8000.0
exact = "w²L = 10²·80 (each run, cut by the miter's plane through the path's corner, holds its section times its length on the path)"
tex = 'w^2 L = 10^2\cdot 80'

[area]
value = 3400.0
exact = "4wL + 2w² = 3200 + 200"
tex = '4wL + 2w^2 = 3200 + 200'
+++

A square bar swept along a path with one sharp corner, mitered there.

The path: two straight runs, from (0, 0, 0) along +x to (40, 0, 0), then turning 120° to the
left (toward +y) and on 40 to (20, 20√3, 0). So the runs meet at (40, 0, 0) at 60°.

The section: a square 10 × 10, square to the path and centered on it, two of its sides
horizontal, at z = −5 and z = 5.

Each run is the square swept straight along it, and at the corner the two are mitered: each is
cut by, and they meet in, the vertical plane through the corner that bisects it,
x + √3y = 40. The miter is a rectangle 20 long and 10 tall in that plane, from the inside of the
corner, at x = 40 − 5√3 ≈ 31.340 on the first run's inner side (y = 5), to its outside, at
x = 40 + 5√3 ≈ 48.660 on the first run's outer side (y = −5). The bar's ends are flat and square
to the path: in the plane x = 0, and in the plane through (20, 20√3, 0) square to the second run.
