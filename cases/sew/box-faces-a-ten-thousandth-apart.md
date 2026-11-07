+++
title = "A box healed from faces a ten-thousandth apart"
tags = ["sew", "heal", "tolerance", "planar", "analytic"]

[volume]
value = 1000.0
exact = "20·10·5"
tex = '20\cdot 10\cdot 5'

[area]
value = 700.0
exact = "2(20·10 + 20·5 + 10·5)"
tex = '2\left(20\cdot 10 + 20\cdot 5 + 10\cdot 5\right)'
+++

Six separate faces, as a file written by a less precise system might hand them over: each lies on
one of the planes x = 0, x = 20, y = 0, y = 10, z = 0 and z = 5 of the box from the origin to
(20, 10, 5), but each is that face of the box shrunk by 0.0001 on every side, so that no two meet.
The face on x = 0, for example, is the rectangle from (0, 0.0001, 0.0001) to (0, 9.9999, 4.9999);
the face on z = 5 is the rectangle from (0.0001, 0.0001, 5) to (19.9999, 9.9999, 5). Where two
faces would meet at an edge of the box, their edges are parallel and about 0.00014 apart.

Sew the six into one closed shell with a tolerance of 0.001, and heal it: each face is extended
along its own plane until it meets its neighbors exactly, so that every edge of the shell lies on
the line where its two faces' planes cross, and every vertex where three planes do. Make the solid
the shell bounds, faced outward.

The solid is the box from the origin to (20, 10, 5). A shell sewn but not healed, its faces left
as they came, falls short of the box's area by about 4·10⁻⁵ of it, more than a pass allows.
