+++
title = "A notched cube with its inside corner rounded"
tags = ["fillet", "concave", "corner", "sphere", "planar", "analytic"]

[volume]
value = 7053.422580176728
exact = "20³ − 10³ + r³(1 − π/6) + 3(1 − π/4)r²(10 − r), r = 3: 7216 − 51.75π"
tex = '20^3 - 10^3 + r^3\left(1 - \tfrac{\pi}{6}\right) + 3\left(1 - \tfrac{\pi}{4}\right)r^2\left(10 - r\right) = 7216 - 51.75\pi,\quad r = 3'

[area]
value = 2365.8915851175015
exact = "6·20² − 3·10² + 3(10 − r)² + 3(π/2)r(10 − r) + πr²/2 + 3(1 − π/4)r², r = 3: 2274 + 29.25π"
tex = '6\cdot 20^2 - 3\cdot 10^2 + 3\left(10 - r\right)^2 + \tfrac{3\pi}{2}r\left(10 - r\right) + \tfrac{\pi r^2}{2} + 3\left(1 - \tfrac{\pi}{4}\right)r^2 = 2274 + 29.25\pi,\quad r = 3'
+++

A cube from the origin to (20, 20, 20) with a notch cut from one corner: the cube from
(10, 10, 10) to (20, 20, 20) removed. The notch's three inside faces, on the planes x = 10,
y = 10 and z = 10, meet in three concave edges, each 10 long, which all start at the inside corner
(10, 10, 10) and run out to the cube's faces x = 20, y = 20 and z = 20.

Round the three concave edges to radius 3, all at once. Each round is a concave quarter cylinder
of radius 3 tangent to the two inside faces it joins, and where all three meet, at the inside
corner, the corner is filled by an eighth of a sphere of radius 3 centered at (13, 13, 13),
tangent to all three faces, which each round runs into tangentially. As a composition: the
material added is every point of the notch that no ball of radius 3 lying inside the notch can
reach.

The result is the notched cube with that material added: along each edge a strip of section
(1 − π/4)·9, and at the corner the cube from (10, 10, 10) to (13, 13, 13) less the eighth of the
ball.
