+++
title = "A wedge a millionth steep"
tags = ["tangent", "boolean", "common", "planar", "near-degenerate", "sliver", "ssi"]
origin = "jyviko/ssi-corpus c7_angle_planes_1e-6"

[volume]
value = 0.000125
exact = "10 · 5 · 5s / 2, s = 10⁻⁶"
tex = '10\cdot 5\cdot \tfrac{5s}{2},\quad s = 10^{-6}'

[area]
value = 100.000075000025
exact = "the top 50 + the lean face 50√(1 + s²) + the end 10·5s + two triangles 2·(5·5s/2), s = 10⁻⁶"
tex = '50 + 50\sqrt{1 + s^2} + 10\cdot 5s + 2\cdot\tfrac{5\cdot 5s}{2},\quad s = 10^{-6}'
+++

A cube of side 10 from (−5, −5, −10) to (5, 5, 0), its top face in the plane z = 0, and the
half-space above the plane z = y·10⁻⁶. Make the solid common to both.

The plane crosses the cube's top face along its middle line, the x axis, at a slope of 10⁻⁶ (an
angle of about 5.7·10⁻⁵ degrees), rising out of the cube toward +y and sinking into it toward −y.
What is common is a wedge under the half of the top face where y < 0: 10 long in x, 5 wide in y,
and 5·10⁻⁶ thick at its end on the face y = −5, thinning to an edge along the x axis. Its faces are
half the cube's top, the plane under it, a strip 10 × 5·10⁻⁶ of the face y = −5, and a sliver
triangle of each of the faces x = ±5.

This case is about two planes that are nearly parallel: where they meet is a well-defined line,
but a kernel that takes planes closer than its tolerance for the same plane over the whole face
finds no wedge, or the whole half. (The wedge lies at z = 0 so that its corners keep their
heights in an STL file's single-precision coordinates.)
