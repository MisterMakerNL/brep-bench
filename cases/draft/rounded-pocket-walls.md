+++
title = "A pocket with rounded corners and its walls drafted"
tags = ["draft", "cone", "pocket", "concave", "fillet", "tangent-chain", "analytic"]

[volume]
value = 14466.913089757878
exact = "abc − (A₀h + P₀th²/2 + πt²h³/3), A₀ = 24·14 − (4 − π)r², P₀ = 76 − (8 − 2π)r, a = 40, b = 30, c = 15, r = 4, h = 10, t = tan 5°"
tex = '\begin{array}{l} abc - \left(A_0 h + \tfrac{1}{2}P_0 th^2 + \tfrac{1}{3}\pi t^2h^3\right),\quad a = 40,\ b = 30,\ c = 15,\ h = 10,\ t = \tan 5^\circ \\ A_0 = 24\cdot 14 - (4 - \pi)r^2,\quad P_0 = 76 - (8 - 2\pi)r,\quad r = 4 \end{array}'

[area]
value = 5158.670564596014
exact = "2(ab + bc + ca) − A₁ + A₀ + (P₀h + πth²)/cos 5°, A₁ = A₀ + P₀th + πt²h², the opening"
tex = '2(ab + bc + ca) - A_1 + A_0 + \frac{P_0 h + \pi th^2}{\cos 5^\circ},\quad A_1 = A_0 + P_0 th + \pi t^2h^2'
+++

The block from the origin to (40, 30, 15), with a pocket cut into its top: the box from
(8, 8, 5) to (32, 22, 15), its four vertical edges rounded to radius 4, open at the block's top
face, its floor at z = 5. The pocket's walls are four planes and four quarter cylinders of radius 4
about the vertical lines through (12, 12), (28, 12), (28, 18) and (12, 18), each tangent to the
planes beside it: concave rounds, seen from the block's material.

Draft all eight walls by 5°, with the pull direction +z and the neutral plane z = 5, the pocket's
floor, so that each leans 5° from the z direction, away from the pocket's middle as it rises. Each
plane turns about its bottom edge; each quarter cylinder becomes a quarter cone about the same
axis, its radius 4 + (z − 5)·tan 5° at height z, so that it stays tangent to the planes beside it.
The pocket's floor and the block's faces are not drafted; the opening in the block's top grows to
meet the drafted walls.

The result is the block less the pocket, whose section at height z is its floor's outline, the
rounded rectangle, moved (z − 5)·tan 5° outward: its straight sides outset by that and its
corners' radii 4 + (z − 5)·tan 5° about the same centres.
