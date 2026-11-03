+++
title = "A boss and the round at its base deleted"
tags = ["edit", "delete-face", "defeature", "fillet", "torus", "cylinder", "planar", "analytic"]

[volume]
value = 14400.0
exact = "60·40·6"
tex = '60\cdot 40\cdot 6'

[area]
value = 6000.0
exact = "2(60·40 + 60·6 + 40·6)"
tex = '2\left(60\cdot 40 + 60\cdot 6 + 40\cdot 6\right)'
+++

A plate, the box from the origin to (60, 40, 6), with a boss on its top: a cylinder of radius 8 on
the vertical axis through (30, 20), from z = 6 to z = 16, joined to the plate. The concave circular
edge where the boss stands on the plate is rounded to radius 2, adding material: the round is a
quarter torus whose centre circle has radius 10 and lies at z = 8, tangent to the plate's top on
the circle of radius 10 and to the boss's side at z = 8.

Then edit the part directly: delete the boss's three faces, its flat top, its side and the round at
its base, and heal the solid by closing the gap they leave with the faces round them. Only the
plate's top touches them, and its plane spans the round's outer circle, so it runs on across it.

The result is the plate alone, the box from the origin to (60, 40, 6).
