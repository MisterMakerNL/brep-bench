+++
title = "A block capped by the Coons patch of four parabolas"
tags = ["fill", "coons", "bezier", "saddle", "planar"]
settled = false
+++

Four parabolic arcs that close a loop over the square from (0, 0) to (10, 10), each a quadratic
Bézier curve in a vertical plane, given by its three control points:

- on y = 0, from (0, 0, 10) through the control point (5, 0, 14) to (10, 0, 10), arching up to
  z = 12 at its middle;
- on x = 10, from (10, 0, 10) through (10, 5, 6) to (10, 10, 10), sagging to z = 8;
- on y = 10, from (10, 10, 10) through (5, 10, 6) to (0, 10, 10), sagging to z = 8;
- on x = 0, from (0, 10, 10) through (0, 5, 14) to (0, 0, 10), arching up to z = 12.

Fill them with a surface: the bilinearly blended Coons patch of the four arcs. With u and v
running from 0 to 1 along the arcs on y = 0 and on x = 0, it is the sum of the blends between
opposite arcs less the blend of the four corners, every blend linear, its blending functions
1 − t and t. Here that is the surface over the square whose height at (10u, 10v) is
10 + 8u(1 − u)(1 − 2v) + 8v(1 − v)(1 − 2u), a saddle. A fill by any other rule makes a different
surface: one that minimizes some energy, or a Coons patch blended by cubic Hermite functions, as
some kernels' Coons fills are.

Make the solid the patch caps: under it, the four vertical faces from z = 0 up to the arcs, on the
planes x = 0, x = 10, y = 0 and y = 10, and the bottom, the square from (0, 0, 0) to (10, 10, 0).

Its volume is 1000, since the patch's rise and fall over the square cancel, but its area, the
saddle's, has no closed form here, so the case is unsettled until two kernels agree.
