+++
title = "A rounded box with its sides drafted, the rounds with them"
tags = ["draft", "cone", "fillet", "tangent-chain", "analytic"]

[volume]
value = 5463.267400412602
exact = "A₀H − P₀tH²/2 + πt²H³/3, A₀ = ab − (4 − π)r², P₀ = 2(a + b) − (8 − 2π)r, a = 30, b = 20, r = 4, H = 10, t = tan 5°"
tex = '\begin{array}{l} A_0 H - \tfrac{1}{2}P_0 tH^2 + \tfrac{1}{3}\pi t^2H^3,\quad H = 10,\ t = \tan 5^\circ \\ A_0 = ab - (4 - \pi)r^2,\quad P_0 = 2(a + b) - (8 - 2\pi)r,\quad a = 30,\ b = 20,\ r = 4 \end{array}'

[area]
value = 2000.7496007461252
exact = "A₀ + (A₀ − P₀tH + πt²H²) + (P₀H − πtH²)/cos 5°, with A₀ and P₀ as for the volume"
tex = 'A_0 + \left(A_0 - P_0 tH + \pi t^2H^2\right) + \frac{P_0 H - \pi tH^2}{\cos 5^\circ},\quad \text{with}\ A_0,\ P_0\ \text{as for the volume}'
+++

The box from the origin to (30, 20, 10), with its four vertical edges rounded to radius 4:
its sides are four planes and four quarter cylinders of radius 4, about the vertical lines through
(4, 4), (26, 4), (26, 16) and (4, 16), each tangent to the planes beside it.

Draft all eight sides by 5°, with the pull direction +z and the neutral plane z = 0, the plane of
its floor, so that each leans 5° from the z direction, inward as it rises. Each plane turns about
its bottom edge. Each quarter cylinder becomes a quarter cone about the same axis, through the same
quarter circle at z = 0, its radius 4 − z·tan 5° at height z, so that it stays tangent to the
drafted planes beside it. The floor and the top are not drafted; the top is trimmed to the drafted
sides.

The result has ten faces. Its section at height z is the floor's outline, the rounded rectangle,
moved z·tan 5° inward: its straight sides inset z·tan 5° and its corners' radii 4 − z·tan 5°
about the same centres. At the top they are inset 10·tan 5° ≈ 0.875, the corners of radius
about 3.125.
