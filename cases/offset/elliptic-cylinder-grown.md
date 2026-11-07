+++
title = "An elliptic cylinder grown by 1"
tags = ["offset", "grow", "ellipse", "canal"]

[volume]
value = 896.45909906955167
exact = "V + Sd + Md² + 4πd³/3 (Steiner), V = πabh, S = 2πab + ph, M = πh + πp/2, p = 4aE(1 − b²/a²), a = 5, b = 3, h = 10, d = 1"
tex = 'V + Sd + Md^2 + \tfrac{4}{3}\pi d^3,\quad V = \pi abh,\ S = 2\pi ab + ph,\ M = \pi h + \tfrac{\pi p}{2},\ p = 4aE\left(1 - \tfrac{b^2}{a^2}\right),\ a = 5,\ b = 3,\ h = 10,\ d = 1'

[area]
value = 505.11142402527668
exact = "S + 2Md + 4πd²"
tex = 'S + 2Md + 4\pi d^2'
+++

The elliptic cylinder of `geometry/elliptic-cylinder`, the ellipse x²/25 + y²/9 ≤ 1 in the plane
z = 0 extruded straight up to z = 10, grown by 1: the solid of every point within 1 of it.

The ends move to z = −1 and z = 11. The side moves out along its normals to the curve at distance
1 outside the ellipse, which is no ellipse (an offset of an ellipse is not a conic, nor even
rational), extruded from z = 0 to z = 10. Each rim becomes the outer quarter of a tube of radius 1
about the rim's ellipse: a canal surface whose spine is an ellipse, not a torus.

E is the complete elliptic integral of the second kind, of parameter m = 1 − b²/a² = 16/25, so p is
the ellipse's perimeter; M is the side's mean curvature integrated over it, πh, plus each rim's
length times half its turning angle, π/4.
