+++
title = "A block capped by the Coons patch of four straight edges"
tags = ["fill", "coons", "ruled", "saddle", "planar", "analytic"]

[volume]
value = 750.0
exact = "∫∫ (5 + xy/10) dx dy over [0, 10]² = 500 + 250"
tex = '\int_0^{10}\!\!\int_0^{10} \left(5 + \tfrac{xy}{10}\right) dx\,dy = 750'

[area]
value = 528.0789275273404
exact = "100 + 2·50 + 2·100 + 100·I, I = ∫∫ √(1 + u² + v²) du dv over [0, 1]² = √3/3 + (2/3)ln(2 + √3) − π/18"
tex = '400 + 100 I,\quad I = \int_0^1\!\!\int_0^1 \sqrt{1 + u^2 + v^2}\,du\,dv = \tfrac{\sqrt{3}}{3} + \tfrac{2}{3}\ln\left(2 + \sqrt{3}\right) - \tfrac{\pi}{18}'
+++

Four straight edges that close a skew quadrilateral, from (0, 0, 5) to (10, 0, 5), to
(10, 10, 15), to (0, 10, 5) and back to (0, 0, 5). Fill them with a surface: the bilinearly blended
Coons patch of the four edges. With u and v running from 0 to 1 along the edges on y = 0 and on
x = 0, it is the sum of the blends between opposite edges less the blend of the four corners, every
blend linear, its blending functions 1 − t and t. (With straight edges it is the hyperbolic
paraboloid z = 5 + xy/10 over the square from (0, 0) to (10, 10), and also the ruled surface
between the edges on y = 0 and y = 10; but build it as a fill of the four edges, not as a loft of
two.) A fill by any other rule makes a different surface: one that minimizes some energy, or a
Coons patch blended by cubic Hermite functions, 1 − 3t² + 2t³ and 3t² − 2t³, as some kernels'
Coons fills are, which here has the same volume under it but not the same area.

Make the solid the patch caps: under it, the four vertical faces from z = 0 up to the edges, on the
planes x = 0, x = 10, y = 0 and y = 10, and the bottom, the square from (0, 0, 0) to (10, 10, 0).
The faces x = 0 and y = 0 are rectangles 5 high; the faces x = 10 and y = 10 are trapezoids rising
from 5 to 15.

The result is the region over that square from z = 0 up to the saddle z = 5 + xy/10.
