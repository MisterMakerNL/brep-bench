+++
title = "A block's round deleted"
tags = ["edit", "delete-face", "fillet", "cylinder", "planar", "analytic"]

[volume]
value = 2000.0
exact = "20·10·10"
tex = '20\cdot 10\cdot 10'

[area]
value = 1000.0
exact = "2(20·10 + 20·10 + 10·10)"
tex = '2\left(20\cdot 10 + 20\cdot 10 + 10\cdot 10\right)'
+++

A block, the box from the origin to (20, 10, 10), with the edge where its top, z = 10, meets its
end, x = 20, rounded to radius 3: the round is a quarter cylinder whose axis is the line x = 17,
z = 7, tangent to the top along x = 17 and to the end along z = 7.

Then edit it directly: delete the round's face, and heal the gap it leaves by extending the faces
around it along their own surfaces. The top runs on to x = 20 and the end up to z = 10, where they
meet in a sharp edge again; the faces y = 0 and y = 10 grow back their corners.

The result is the box from the origin to (20, 10, 10).
