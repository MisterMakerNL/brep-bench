+++
title = "Three bodies nested in each other's hollows"
tags = ["boolean", "cut", "union", "analytic", "lumps"]

[volume]
value = 2714.336052701581
exact = "4π(10³ − 8³ + 6³ − 4³ + 2³)/3"
tex = '\tfrac{4}{3}\pi\left(10^3 - 8^3 + 6^3 - 4^3 + 2^3\right)'

[area]
value = 2764.601535159018
exact = "4π(10² + 8² + 6² + 4² + 2²)"
tex = '4\pi\left(10^2 + 8^2 + 6^2 + 4^2 + 2^2\right)'
+++

Five balls, none of them sharing a center:

- of radius 10, centered on the origin;
- of radius 8, centered at (1, 0, 0);
- of radius 6, centered at (1, 1, 0);
- of radius 4, centered at (1, 1, 1);
- of radius 2, centered at (1, 1.5, 1.5).

Each lies inside the one before it without touching it. The solid is the first ball less the
second, joined with the third less the fourth, joined with the fifth: three bodies, not touching,
each inside the hollow of the one around it. A hollow ball holds a hollow ball, which holds a ball.
