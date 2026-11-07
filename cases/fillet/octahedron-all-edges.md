+++
title = "An octahedron with every edge rounded, four rounds meeting at each corner"
tags = ["fillet", "corner", "sphere", "planar", "analytic"]

[volume]
value = 1270.7657089174301
exact = "V′ + S′r + M′r² + 4πr³/3 (Steiner) over the octahedron shrunk by r, |x| + |y| + |z| ≤ s′, s′ = 10 − r√3, r = 2: 4s′³/3 + 8√3s′² + 24√2 s′·arccos(1/3) + 32π/3"
tex = '\tfrac{4}{3}{s^{\prime}}^3 + 8\sqrt{3}\,{s^{\prime}}^2 + 24\sqrt{2}\,s^{\prime}\arccos\tfrac{1}{3} + \tfrac{32\pi}{3},\quad s^{\prime} = 10 - 2\sqrt{3}'

[area]
value = 619.29504650969118
exact = "S′ + 2M′r + 4πr²: 4√3s′² + 24√2 s′·arccos(1/3) + 16π"
tex = '4\sqrt{3}\,{s^{\prime}}^2 + 24\sqrt{2}\,s^{\prime}\arccos\tfrac{1}{3} + 16\pi,\quad s^{\prime} = 10 - 2\sqrt{3}'
+++

The octahedron |x| + |y| + |z| ≤ 10: its six corners on the axes, 10 from the origin, such as
(10, 0, 0), and its eight faces equilateral triangles, such as the one in the plane
x + y + z = 10. Round all twelve of its edges to radius 2. Four rounds meet at each corner.

Each round is the cylinder of radius 2 a ball of that radius sweeps as it rolls along the edge
touching both faces. At each corner, the ball that touches all four faces there closes the corner
with a patch of its sphere, which all four rounds run into tangentially: the octahedron has an
inscribed sphere, so the four planes 2 inside a corner's faces meet in one point, the patch's
center (at the corner (10, 0, 0), the point (10 − 2√3, 0, 0)). As a composition: the solid is the
union of every ball of radius 2 that lies inside the octahedron, which is the octahedron shrunk by
2, |x| + |y| + |z| ≤ 10 − 2√3, grown by 2.

It is `chamfer/octahedron-all-edges` with its edges rounded instead of chamfered: the six corners
are each a square patch of a sphere, bounded by four rounds, not a flat square.

M′ is the shrunk octahedron's twelve edges' lengths, s′√2 each, times half the angle each turns
through, arccos(1/3).
