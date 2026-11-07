+++
title = "A NURBS ball grown"
tags = ["offset", "grow", "nurbs", "rational", "pole", "degenerate-edge"]

[volume]
value = 1436.755040241732
exact = "4π(r + d)³/3, r = 5, d = 2"
tex = '\tfrac{4}{3}\pi\left(r + d\right)^3,\quad r = 5,\ d = 2'

[area]
value = 615.7521601035994
exact = "4π(r + d)²"
tex = '4\pi\left(r + d\right)^2'
+++

A ball of radius 5 centered on the origin, its whole surface one rational B-spline surface (a
NURBS), the standard biquadratic sphere of `geometry/nurbs-sphere`: a 9 × 5 net whose first and
last rows each collapse to a single point, a pole.

The surface is the tensor product of two rational quadratic curves. Around the axis (u), the unit
circle of nine control points cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1),
(1, −1), (1, 0), with weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots
(0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1). Along the meridian (v), the semicircle from the south pole to
the north of five control points (ρⱼ, zⱼ) = (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights
vⱼ = 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1). The net's control point (i, j) is
(ρⱼ·cᵢ, zⱼ) = (ρⱼ cᵢₓ, ρⱼ cᵢᵧ, zⱼ), with weight wᵢvⱼ. Build the ball from that one face, closed on
its seam and at its poles; do not use a sphere.

Then grow the ball by 2: offset its NURBS face outward along its normal by 2, the offset running
through the poles, where the face's parameters pinch. Do not replace the face by a sphere before
offsetting it.

The result is the ball of radius 7 centered on the origin.
