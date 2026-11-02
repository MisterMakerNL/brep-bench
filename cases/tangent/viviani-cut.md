+++
title = "A ball bored by a cylinder tangent inside it, pinched where they touch"
tags = ["tangent", "boolean", "cut", "ssi", "singular-curve"]

[volume]
value = 2983.2839912820846
exact = "4πR³/3 − V₀ = 2000π/3 + 8000/9, R = 10, V₀ = 2000π/3 − 8000/9 the common part (tangent/viviani-common)"
tex = '\tfrac{4\pi R^3}{3} - V_0 = \tfrac{2000\pi}{3} + \tfrac{8000}{9},\quad V_0 = \tfrac{2000\pi}{3} - \tfrac{8000}{9},\ R = 10'

[area]
value = 1428.3185307179585
exact = "2πR² + 8R² = 200π + 800: the sphere outside the cylinder, 4πR² − 2R²(π − 2), and the wall inside the ball, 4R², R = 10 (tangent/viviani-common)"
tex = '\left(4\pi R^2 - 2R^2(\pi - 2)\right) + 4R^2 = 2\pi R^2 + 8R^2 = 200\pi + 800,\quad R = 10'
+++

A ball of radius 10 centred on the origin, bored by a solid cylinder of radius 5 whose axis is the
line x = 5, y = 0, parallel to the z axis, running from z = −12 to z = 12. Cut the cylinder from
the ball.

The cylinder's radius is half the ball's and its wall passes through the ball's centre, so it
touches the ball's sphere from inside at one point, (10, 0, 0), where the two surfaces are
tangent. The hole runs through the ball from top to bottom, and at z = 0 it reaches the sphere at
that point: there the ball's section is a crescent whose horns close on it.

The sphere and the wall meet in Viviani's curve, (10 cos²t, 10 cos t sin t, 10 sin t): a figure
eight on the sphere that crosses itself at (10, 0, 0), and passes through (0, 0, 10) and
(0, 0, −10). Round the crossing point what is left is two slivers, one towards +y and one towards
−y, each between the wall and the sphere, thinning to nothing as they reach it; above and below
it nothing is left. The result is one solid, pinched to that point: the slivers touch there and
nowhere else, and its surface is not a manifold at that point.

If the ball you make has poles and a seam, keep them off the curve and away from its crossing:
put the poles on the y axis, at (0, ±10, 0), and the seam through (−10, 0, 0). If the cylinder you
make has a seam, put it on the line x = 0, y = 0, the side of its wall away from (10, 0, 0).
