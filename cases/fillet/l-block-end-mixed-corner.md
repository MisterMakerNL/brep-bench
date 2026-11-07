+++
title = "An L-block's end rounded across its inside corner, one concave round and two convex"
tags = ["fillet", "corner", "concave", "torus", "planar", "analytic"]

[volume]
value = 2024.683159826111
exact = "2040 + 4(1 − π/4)·10 − 2·12·4(1 − π/4) − (π/2)(2 + d)·4(1 − π/4) (Pappus), d = 2(10 − 3π)/(12 − 3π): 1984 + 14π − (π/2)(2 + d)(4 − π)"
tex = '1984 + 14\pi - \tfrac{\pi}{2}\left(2 + d\right)\left(4 - \pi\right),\quad d = \frac{2\left(10 - 3\pi\right)}{12 - 3\pi}'

[area]
value = 1176.562210449103
exact = "1208 + (−32 + 8π) + (−104 + 19π + 2π²) (Pappus): 1072 + 27π + 2π²"
tex = '1072 + 27\pi + 2\pi^2'
+++

An L-block, the union of the box from the origin to (20, 10, 6), its foot, and the box from the
origin to (6, 10, 20), its upright. Where the foot's top, z = 6, meets the upright's face x = 6,
they make a concave edge along y, from (6, 0, 6) to (6, 10, 6). At its end y = 0 the concave edge
meets two convex edges of the end face y = 0: the one up the upright, from (6, 0, 6) to
(6, 0, 20), and the one along the foot, from (6, 0, 6) to (20, 0, 6). Round all three to radius 2.
One concave and two convex rounds meet at the corner (6, 0, 6).

Rounds of opposite convexity meeting at a corner can be closed more than one way, so the corner is
the one these rounds make one after another:

1. The concave edge is rounded to radius 2: a concave quarter cylinder about the line x = 8, z = 8,
   tangent to the upright along z = 8 and to the foot along x = 8, which ends square on the end
   faces y = 0 and y = 10. That turns the end face's edge into one chain, tangent all along: up
   the upright from (6, 0, 20) to (6, 0, 8), round the quarter circle of radius 2 about (8, 0, 8)
   to (8, 0, 6), and along the foot to (20, 0, 6).
2. That whole chain is rounded to radius 2, convex: along its straight parts a quarter cylinder,
   and around its quarter circle the inner quarter of a torus's tube, the tube of radius 2 about
   the circle of radius 4 about the line x = 8, z = 8 in the plane y = 2. The chain's ends run out
   square onto the upright's top z = 20 and the foot's end x = 20.

The rounds at the corner are therefore a quarter of a torus, a convex round turned around a
concave one. (Rounded all at once, a kernel's corner may differ.) d is the distance from the
corner of the convex round's section to its centroid.
