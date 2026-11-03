+++
title = "A chamfer deleted where a hole comes out through it"
tags = ["edit", "delete-face", "defeature", "chamfer", "hole", "cylinder", "planar", "analytic"]

[volume]
value = 15748.672587712817
exact = "40·20·20 − π·2²·20 = 16000 − 80π"
tex = '40\cdot 20\cdot 20 - \pi\cdot 2^2\cdot 20 = 16000 - 80\pi'

[area]
value = 4226.194671058465
exact = "2(40·20 + 40·20 + 20·20) − 2π·2² + 2π·2·20 = 4000 + 72π"
tex = '2\left(40\cdot 20 + 40\cdot 20 + 20\cdot 20\right) - 2\pi\cdot 2^2 + 2\pi\cdot 2\cdot 20 = 4000 + 72\pi'
+++

A block, the box from the origin to (40, 20, 20), with the edge where its top, z = 20, meets its
back, y = 20, chamfered 6 along both faces: the chamfer is the plane y + z = 34, from the line
y = 14 on the top to the line z = 14 on the back.

Then drill it right through along z by a hole of radius 2 whose axis is the vertical line through
(20, 17). The hole enters through the bottom in a circle and comes out through the chamfer, not the
top: across the hole, 15 ≤ y ≤ 19, the chamfer lies between z = 15 and z = 19. So the hole's wall
runs from the bottom up to an ellipse on the chamfer.

Then edit the part directly: delete the chamfer, and heal the solid by closing the gap it leaves
with the faces round it. The top and the back run on until they meet in the sharp edge y = z = 20;
the ends x = 0 and x = 40 run on to that edge too; and the hole's wall runs on up to the top, where
it now comes out in a circle.

The result is the box from the origin to (40, 20, 20), drilled right through along z by the hole of
radius 2 through (20, 17).
