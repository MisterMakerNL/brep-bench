+++
title = "A block's edge blended by a parabolic conic"
tags = ["fillet", "conic", "planar", "analytic"]

[volume]
value = 1985.0
exact = "20·10·10 − 10·(3·3/2 − (2/3)(3·3/2)): the corner's triangle less the parabolic segment, 1.5, along 10"
tex = '20\cdot 10\cdot 10 - 10\left(\tfrac{9}{2} - \tfrac{2}{3}\cdot\tfrac{9}{2}\right) = 1985'

[area]
value = 985.696757204207
exact = "1000 − 2·3·10 − 2·1.5 + 10L, L = 3 + (3√2/2)ln(1 + √2), the parabola's length: 967 + 15√2 ln(1 + √2)"
tex = '1000 - 2\cdot 3\cdot 10 - 2\cdot 1.5 + 10L,\quad L = 3 + \tfrac{3\sqrt{2}}{2}\ln\left(1 + \sqrt{2}\right):\ 967 + 15\sqrt{2}\,\ln\left(1 + \sqrt{2}\right)'
+++

A block, the box from the origin to (20, 10, 10). Blend the edge where its top, z = 10, meets its
end, x = 20, with a conic blend of rho 0.5, a parabola, tangent to the top and to the end each 3
from the edge.

The blend's section, the same in every plane y = c, is the parabola tangent to the top at
(17, c, 10) and to the end at (20, c, 7): the quadratic Bézier curve whose control points are those
two points and the edge's point between them, (20, c, 10). (A conic blend's rho is where the conic
crosses the line from the edge's point to the middle of the chord between the two tangent points,
as a fraction of the way from the chord: 0.5 makes it a parabola, as here; a circular round, which
this is not, has rho √2 − 1 ≈ 0.414.) The blend is the surface the parabola sweeps along the edge,
from y = 0 to y = 10, and the corner outside it is removed.

The result is the block with that edge blended: its top now ends at x = 17, its end at z = 7, and
the faces y = 0 and y = 10 each lose the region between the corner and the parabola, of area 1.5.
