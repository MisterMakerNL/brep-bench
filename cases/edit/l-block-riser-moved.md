+++
title = "An L-block's step moved out"
tags = ["edit", "move-face", "planar", "concave", "analytic"]

[volume]
value = 18000.0
exact = "(40·10 + 25·20)·20"
tex = '\left(40\cdot 10 + 25\cdot 20\right)\cdot 20'

[area]
value = 4600.0
exact = "2(40·10 + 25·20) + (40 + 10 + 15 + 20 + 25 + 30)·20"
tex = '2\left(40\cdot 10 + 25\cdot 20\right) + \left(40 + 10 + 15 + 20 + 25 + 30\right)\cdot 20'
+++

An L-block: the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (15, 10), (15, 30), (0, 30) in
the plane y = 0, extruded along +y to y = 20. So it has a foot 40 long and 10 tall, and an upright
15 thick and 30 tall at its end x = 0. The upright's face x = 15, from z = 10 to z = 30, is the
riser; it meets the foot's top, z = 10, in a concave edge.

Then edit the block directly: move the riser out by 10, along +x, to the plane x = 25. The faces
beside it follow it: the foot's top is cut back to run from x = 25 to x = 40, the upright's top
runs on out to x = 25, and the two sides y = 0 and y = 20 are reshaped to match.

The result is the L-block whose section is the polygon through (0, 0), (40, 0), (40, 10), (25, 10),
(25, 30), (0, 30), from y = 0 to y = 20.
