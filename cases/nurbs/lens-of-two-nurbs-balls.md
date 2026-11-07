+++
title = "The lens common to two NURBS balls"
tags = ["nurbs", "rational", "boolean", "common", "pole", "seam"]

[volume]
value = 108.90854532444617
exact = "2·πc²(3R − c)/3, R = 5, c = R − 3 = 2: 104π/3"
tex = '2\cdot\tfrac{\pi c^2 (3R - c)}{3} = \tfrac{104\pi}{3},\quad R = 5,\ c = R - 3 = 2'

[area]
value = 125.66370614359173
exact = "2·2πRc = 40π"
tex = '2\cdot 2\pi R c = 40\pi'
+++

Two balls of radius 5, centered at (−3, 0, 0) and (3, 0, 0), each bounded by one rational B-spline
surface (a NURBS), the standard biquadratic sphere of `geometry/nurbs-sphere` moved to its center:
a 9 × 5 net whose first and last rows each collapse to a pole.

Each surface is the tensor product of two rational quadratic curves. Around the axis (u), the unit
circle of nine control points cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1),
(1, −1), (1, 0), with weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots
(0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1). Along the meridian (v), the semicircle from the south pole to
the north of five control points (ρⱼ, zⱼ) = (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights
vⱼ = 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1). The net's control point (i, j) is
the center plus (ρⱼ cᵢₓ, ρⱼ cᵢᵧ, zⱼ), with weight wᵢvⱼ. So each ball's poles are 5 above and below
its center, and its seam is the meridian in the half plane y = 0 on the +x side of its center. Do
not use spheres.

Their common part is a lens. The two surfaces meet in the circle of radius 4 about the x axis in
the plane x = 0, which crosses the first ball's seam at (0, 0, ±4): every point of the intersection
lies on two freeform faces, neither a plane nor a cylinder.

The result is the lens: two caps of height 2, each cut from a ball of radius 5, base to base.
