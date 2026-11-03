+++
title = "A counterbored blind hole deleted"
tags = ["edit", "delete-face", "hole", "counterbore", "cone", "cylinder", "planar", "analytic"]

[volume]
value = 32000.0
exact = "40·40·20"
tex = '40\cdot 40\cdot 20'

[area]
value = 6400.0
exact = "2(40·40 + 40·20 + 40·20)"
tex = '2\left(40\cdot 40 + 40\cdot 20 + 40\cdot 20\right)'
+++

A block, the box from the origin to (40, 40, 20), with a counterbored blind hole drilled down into
its top on the vertical axis through (20, 20):

- the counterbore: radius 8, from the top, z = 20, down to its flat floor at z = 15;
- the hole: radius 4, from the counterbore's floor down to z = 5;
- the drill's point: a cone below the hole, with an included angle of 118°, its rim the hole's
  bottom circle at z = 5 and its tip on the axis 4/tan 59° ≈ 2.403 lower.

So the hole has four faces: the counterbore's wall, the ring of its floor, the hole's wall and the
point's cone. They meet edge to edge, each pair of the coaxial cylinders and cone sharing a circle.

Then edit the block directly: delete all four of the hole's faces, and heal the solid by closing
the gap they leave with the faces round them. Only the top touches them, and its plane spans the
counterbore's mouth, so it runs on across it.

The result is the undrilled block, the box from the origin to (40, 40, 20).
