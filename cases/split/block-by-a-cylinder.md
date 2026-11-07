+++
title = "A block split by a cylindrical sheet"
tags = ["split", "cylinder", "planar", "lumps", "analytic"]

[volume]
value = 4000.0
exact = "20·20·10"
tex = '20\cdot 20\cdot 10'

[area]
value = 2353.9822368615505
exact = "2(20·20) + 4(20·10) + 2·2π·6·10 = 1600 + 240π"
tex = '2\left(20\cdot 20\right) + 4\left(20\cdot 10\right) + 2\cdot 2\pi\cdot 6\cdot 10 = 1600 + 240\pi'
+++

A block, the box from the origin to (20, 20, 10), and a sheet: the cylindrical surface of radius 6
about the vertical line through (10, 10), with no faces at its ends, running from z = −1 to
z = 11, past the block's top and bottom.

Split the block with the sheet, keeping both pieces: the round post inside the sheet, a cylinder
of radius 6 from z = 0 to z = 10, and the block outside it, holed right through. Each piece gets
its own face where the sheet cut it. Then move the post up by 20, along +z, so that the two pieces
no longer touch. Do not cut the block twice with a solid cylinder: the case is about one split
that keeps every piece.

The result is two bodies: the box from the origin to (20, 20, 10) with a hole of radius 6 through
it along the line through (10, 10), and the cylinder of radius 6 about that line from z = 20 to
z = 30.
