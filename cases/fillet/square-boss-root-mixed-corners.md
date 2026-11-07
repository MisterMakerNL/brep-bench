+++
title = "A square boss rounded at its corners and its root, two concave rounds and one convex at each corner"
tags = ["fillet", "corner", "concave", "torus", "planar", "analytic"]

[volume]
value = 8535.669714402615
exact = "8560 − 4·10·9(1 − π/4) + 40·4(1 − π/4) + 2π(3 + d)·4(1 − π/4) (Pappus), d = 2(10 − 3π)/(12 − 3π): 8360 + 50π + 2π(3 + d)(4 − π)"
tex = '8360 + 50\pi + 2\pi\left(3 + d\right)\left(4 - \pi\right),\quad d = \frac{2\left(10 - 3\pi\right)}{12 - 3\pi}'

[area]
value = 3639.7579738406407
exact = "3740 + (−240 + 60π) + (−160 + 4π + 10π²) (Pappus): 3340 + 64π + 10π²"
tex = '3340 + 64\pi + 10\pi^2'
+++

A plate, the box from the origin to (40, 30, 5), with a square boss joined to its top: the box from
(12, 7, 5) to (28, 23, 15). The boss's four upright edges are convex; where its sides meet the
plate's top, z = 5, they make four concave edges, its root. At each of the root's corners one
convex edge meets two concave ones. Round the upright edges to radius 3 and the root to radius 2.

Rounds of opposite convexity meeting at a corner can be closed more than one way, so the corners
are the ones these rounds make one after another:

1. The four upright edges are rounded to radius 3: each a convex quarter cylinder about an upright
   line 3 inside the boss's corner, such as x = 15, y = 10, from z = 5 to z = 15. That makes the
   root one closed chain, tangent all along: a square of side 16 with its corners rounded to
   radius 3.
2. That whole chain is rounded to radius 2, concave: along its straight parts a concave quarter
   cylinder, and around each rounded corner a quarter of the outer part of a torus's tube, the
   tube of radius 2 about the circle of radius 5, in the plane z = 7, about the upright round's
   axis.

The boss's top is left sharp. d is the distance from the corner of the concave round's section to
its centroid.
