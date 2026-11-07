+++
title = "An elliptic cylinder shrunk by 1"
tags = ["offset", "shrink", "ellipse"]

[volume]
value = 197.90786875230851
exact = "(πab − pd + πd²)(h − 2d), p = 4aE(1 − b²/a²), a = 5, b = 3, h = 10, d = 1"
tex = '\left(\pi ab - pd + \pi d^2\right)\left(h - 2d\right),\quad p = 4aE\left(1 - \tfrac{b^2}{a^2}\right),\ a = 5,\ b = 3,\ h = 10,\ d = 1'

[area]
value = 203.42747563782546
exact = "2(πab − pd + πd²) + (p − 2πd)(h − 2d)"
tex = '2\left(\pi ab - pd + \pi d^2\right) + \left(p - 2\pi d\right)\left(h - 2d\right)'
+++

The elliptic cylinder of `geometry/elliptic-cylinder`, the ellipse x²/25 + y²/9 ≤ 1 in the plane
z = 0 extruded straight up to z = 10, shrunk by 1: the solid of every point at least 1 from its
outside.

The ends move to z = 1 and z = 9, and the edges stay sharp. The side moves in along its normals to
the curve at distance 1 inside the ellipse, which is no ellipse, extruded from z = 1 to z = 9. The
ellipse curves most tightly at the ends of its long axis, (±5, 0), with radius of curvature
b²/a = 1.8; moved in by 1, less than that, the curve stays smooth, and the area inside it and its
length follow from the ellipse's own (Steiner): πab − pd + πd² and p − 2πd.

E is the complete elliptic integral of the second kind, of parameter m = 1 − b²/a² = 16/25, so p is
the ellipse's perimeter.
