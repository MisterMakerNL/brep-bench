+++
title = "A pyramid on a rectangle with every edge rounded, its apex split"
tags = ["fillet", "corner", "sphere", "planar", "analytic"]

[volume]
value = 848.69097216022387
exact = "V′ + S′r + M′r² + 4πr³/3 (Steiner), r = 3/2, over K′, the pyramid shrunk by r: a hip roof on the rectangle 2a′ × 2b′ at z = r, a′ = (35 − √61)/4, b′ = (21 − 3√5)/4, its ridge along x at z = 12 − 3√5/2 from x = −x′ to x′, x′ = (5√5 − √61)/4; V′ = (2/3)Hb′(2a′ + x′), H = (21 − 3√5)/2; M′ = Σ each edge's length × half its turning angle: arccos(−1/√5) on the long base edges, arccos(−5/√61) on the short, arccos(5/√305) on the four hips, arccos(−3/5) on the ridge"
tex = 'V^{\prime} + S^{\prime}r + M^{\prime}r^2 + \tfrac{4}{3}\pi r^3,\quad r = \tfrac{3}{2},\ V^{\prime} = \tfrac{2}{3}Hb^{\prime}\left(2a^{\prime} + x^{\prime}\right),\ a^{\prime} = \tfrac{35 - \sqrt{61}}{4},\ b^{\prime} = \tfrac{21 - 3\sqrt{5}}{4},\ x^{\prime} = \tfrac{5\sqrt{5} - \sqrt{61}}{4},\ H = \tfrac{21 - 3\sqrt{5}}{2}'

[area]
value = 527.66072666645737
exact = "S′ + 2M′r + 4πr², S′ the hip roof's area: 4a′b′ + 2b′√((a′ − x′)² + H²) + 2(a′ + x′)√(b′² + H²)"
tex = 'S^{\prime} + 2M^{\prime}r + 4\pi r^2,\quad S^{\prime} = 4a^{\prime}b^{\prime} + 2b^{\prime}\sqrt{\left(a^{\prime} - x^{\prime}\right)^2 + H^2} + 2\left(a^{\prime} + x^{\prime}\right)\sqrt{{b^{\prime}}^2 + H^2}'
+++

A pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0) in the plane z = 0, its apex at
(0, 0, 12): five faces, the base and four triangles, and eight edges. Round all eight edges to
radius 3/2.

Each round is the cylinder of radius 3/2 a ball of that radius sweeps as it rolls along the edge
touching both faces. At each corner of the base three faces meet, and the ball touching all three
closes the corner with a patch of its sphere. At the apex four faces meet, but no ball touches
all four: the rectangle has no inscribed circle, so the pyramid has no inscribed sphere, and the
planes 3/2 inside the two long sides (the faces through the edges on y = ±6) meet lower, at
z = 12 − 3√5/2, than the planes 3/2 inside the two short ends do. So the apex splits. A ball
rolling up between the long sides touches them both along a short segment where the pyramid has
no edge, the line y = 0, z = 12 − 3√5/2 from x = −x′ to x′, x′ = (5√5 − √61)/4 ≈ 0.84, and sweeps
a round of radius 3/2 about it between the two long sides; at each end of that segment, where the
ball also touches a short end, a patch of its sphere closes the round, and the hips' rounds run
into those two patches.

As a composition: the solid is the union of every ball of radius 3/2 that lies inside the
pyramid, which is the pyramid shrunk by 3/2 (a hip roof, its ridge that short segment) grown by
3/2. (Chamfered instead, the apex has no flat face: see `chamfer/octahedron-all-edges`.)
