+++
title = "A box stood on part of another, joined, twenty ways"
tags = ["fuzz", "boolean", "union", "coplanar"]

[family]
require = ["ox > 0.05*min(a2, b2)", "oy > 0.05*min(a2, b2)"]

[params]
a1 = [5, 30]
b1 = [5, 30]
c1 = [2, 20]
a2 = [3, 25]
b2 = [3, 25]
c2 = [2, 20]
x2 = ["-0.8*a2", "a1 - 0.2*a2"]
y2 = ["-0.8*b2", "b1 - 0.2*b2"]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[let]
ox = "min(a1, x2 + a2) - max(0, x2)"
oy = "min(b1, y2 + b2) - max(0, y2)"

[volume]
formula = "a1*b1*c1 + a2*b2*c2"
exact = "a₁b₁c₁ + a₂b₂c₂"
tex = 'a_1 b_1 c_1 + a_2 b_2 c_2'

[area]
formula = "2*(a1*b1 + b1*c1 + c1*a1) + 2*(a2*b2 + b2*c2 + c2*a2) - 2*ox*oy"
exact = "2(a₁b₁ + b₁c₁ + c₁a₁) + 2(a₂b₂ + b₂c₂ + c₂a₂) − 2·o_x·o_y, the shared rectangle o_x by o_y"
tex = '2(a_1b_1 + b_1c_1 + c_1a_1) + 2(a_2b_2 + b_2c_2 + c_2a_2) - 2\,o_x o_y'
+++

A box from the origin to (`a1`, `b1`, `c1`), and a second box from (`x2`, `y2`, `c1`) to
(`x2` + `a2`, `y2` + `b2`, `c1` + `c2`), standing on the first's top face. The second's bottom face
and the first's top face lie in one plane and overlap in a rectangle, which may run past either's
edges. Join them.

Then turn the solid by `psi` about the axis through the origin with direction (sin `theta` cos
`phi`, sin `theta` sin `phi`, cos `theta`), right-handed.

`boolean/boxes-sharing-part-of-a-face`, of any proportions and overlap, in any orientation.
