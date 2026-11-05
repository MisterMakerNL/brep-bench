+++
title = "A round pipe mitered along three edges of a cube"
tags = ["sweep", "space-path", "corner", "miter", "analytic"]

[volume]
value = 1130.9733552923256
exact = "πr²L, r = 2, L = 3·30 = 90: = 360π (each run between its miters' planes holds πr² times its length on the path)"
tex = '\pi r^2 L,\quad r = 2,\ L = 3\cdot 30 = 90:\ = 360\pi'

[area]
value = 1156.1060965210438
exact = "2πrL + 2πr² = 360π + 8π"
tex = '2\pi r L + 2\pi r^2 = 360\pi + 8\pi'
+++

A pipe of circular section swept along three edges of a cube, mitered at its two corners.

The path: three straight runs along three edges of a cube of side 30, from (0, 0, 0) to
(30, 0, 0), to (30, 30, 0), to (30, 30, 30). Each corner is a right angle, and the second turns
out of the plane of the first two runs.

The section: a disk of radius 2, square to the path and centered on it.

Each run is the disk swept straight along it, and at each corner the two runs are mitered: each
is cut by, and they meet in, the plane through the corner that bisects it, square to the sum of
the two runs' directions. At (30, 0, 0) that is the plane x + y = 30, and at (30, 30, 0) the
plane y + z = 30; each miter is an ellipse in its plane, with half-axes 2 and 2√2. The pipe's
ends are flat and square to the path, at (0, 0, 0) and at (30, 30, 30).
