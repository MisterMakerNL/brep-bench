+++
title = "A blind drilled hole deleted, point and all"
tags = ["edit", "delete-face", "hole", "cylinder", "cone", "planar", "analytic"]

[volume]
value = 24000.0
exact = "40·30·20"
tex = '40\cdot 30\cdot 20'

[area]
value = 5200.0
exact = "2(40·30 + 40·20 + 30·20)"
tex = '2\left(40\cdot 30 + 40\cdot 20 + 30\cdot 20\right)'
+++

A block, the box from the origin to (40, 30, 20), with a blind hole drilled into its top along the
vertical line through (20, 15). The hole's wall is the cylinder of radius 4 about that line from
the top, z = 20, down to z = 10, and the hole ends in a drill point: the cone about the same line
with its apex at (20, 15, 6), its half angle 45°, meeting the wall in the circle of radius 4 at
z = 10.

Then edit it directly: delete the hole, both its wall and its point, and heal the gap they leave by
extending the faces around them along their own surfaces. The top closes over the hole's circle.

The result is the box from the origin to (40, 30, 20).
