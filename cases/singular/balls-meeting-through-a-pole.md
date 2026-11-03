+++
title = "Two balls meeting through a pole of each"
tags = ["singular", "pole", "boolean", "union", "sphere", "analytic"]

[volume]
value = 938.2890058721515
exact = "2·4πr³/3 − lens, the lens two caps of height r − d/2, r = 5, d = 6"
tex = '2\cdot\tfrac{4}{3}\pi r^3 - 2\cdot\tfrac{\pi}{3}h^2(3r - h),\quad h = r - \tfrac{d}{2},\ r = 5,\ d = 6'

[area]
value = 502.6548245743669
exact = "2·4πr² − 2·2πr(r − d/2), r = 5, d = 6"
tex = '2\cdot 4\pi r^2 - 2\cdot 2\pi r\left(r - \tfrac{d}{2}\right),\quad r = 5,\ d = 6'
+++

Two balls of radius 5, one centred on the origin and one on (4.8, 0, 3.6). Make their union.

The centres are 6 apart, as in `boolean/spheres-union`, so the result is the same solid turned:
the spheres meet in a circle of radius 4 about the point halfway between the centres, square to
the line through them.

This case is about that circle running through a pole of each ball. If the balls you make have
poles, put each ball's on the vertical through its centre: the first's at (0, 0, ±5) and the
second's at (4.8, 0, 8.6) and (4.8, 0, −1.4). The circle then passes through (0, 0, 5), a pole of
the first, and through (4.8, 0, −1.4), a pole of the second.
