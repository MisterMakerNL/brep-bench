+++
title = "A plate's hole deleted"
tags = ["edit", "delete-face", "hole", "planar", "cylinder", "analytic"]

[volume]
value = 9600.0
exact = "40·30·8"
tex = '40\cdot 30\cdot 8'

[area]
value = 3520.0
exact = "2(40·30 + 40·8 + 30·8)"
tex = '2\left(40\cdot 30 + 40\cdot 8 + 30\cdot 8\right)'
+++

A plate, the box from the origin to (40, 30, 8), drilled right through along z by a hole of
radius 5 whose axis is the vertical line through (20, 15). The hole's wall is one cylindrical face,
and the plate's top and bottom each have a circular edge round it.

Then edit the drilled plate directly: delete the hole's wall, and heal the solid by closing the
gap it leaves with the faces round it. The top and bottom are planes that already span the hole,
so each runs on across it and its circular edge goes.

The result is the undrilled plate, the box from the origin to (40, 30, 8).
