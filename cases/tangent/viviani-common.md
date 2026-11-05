+++
title = "Viviani's solid: a ball and a cylinder tangent inside it, their common part"
tags = ["tangent", "boolean", "common", "ssi", "singular-curve"]

[volume]
value = 1205.5062135043065
exact = "2R³(π − 4/3)/3 = 2000π/3 − 8000/9, R = 10: over the cylinder's disk r ≤ R cos θ, ∫∫ 2√(R² − r²) r dr dθ = (2R³/3)∫(1 − |sin θ|³) dθ"
tex = '\tfrac{2R^3}{3}\left(\pi - \tfrac{4}{3}\right) = \tfrac{2000\pi}{3} - \tfrac{8000}{9},\quad R = 10'

[area]
value = 628.3185307179587
exact = "2πR² = 200π, R = 10: the ball's sphere inside the cylinder, 2R²(π − 2), and the cylinder's wall inside the ball, ∫ 2R|sin(φ/2)|·(R/2) dφ over a turn = 4R²"
tex = '2R^2(\pi - 2) + 4R^2 = 2\pi R^2 = 200\pi,\quad R = 10'
+++

A ball of radius 10 centered on the origin, and a solid cylinder of radius 5 whose axis is the line
x = 5, y = 0, parallel to the z axis, running from z = −12 to z = 12. Make the solid common to
both.

The cylinder's radius is half the ball's and its wall passes through the ball's center, so it
touches the ball's sphere from inside at one point, (10, 0, 0), where the two surfaces are
tangent. Elsewhere the wall leaves the ball: its ends, at z = ±12, are outside it. The common part
is the stretch of the cylinder inside the ball, capped above and below by the sphere.

The sphere and the wall meet in Viviani's curve, (10 cos²t, 10 cos t sin t, 10 sin t): a figure
eight on the sphere that crosses itself at (10, 0, 0), and passes through (0, 0, 10) and
(0, 0, −10). Around the crossing point the surface is four wedges, alternating: the cylinder's
wall toward +y and toward −y, and the sphere above and below. The solid is convex, and its
surface is a manifold there.

If the ball you make has poles and a seam, keep them off the curve and away from its crossing:
put the poles on the y axis, at (0, ±10, 0), and the seam through (−10, 0, 0). If the cylinder you
make has a seam, put it on the line x = 0, y = 0, the side of its wall away from (10, 0, 0).
