+++
title = "A post extruded up to the inside of a hollow dome"
tags = ["feature", "extrude", "up-to-face", "sphere", "void", "analytic"]

[volume]
value = 7559.30988769618
exact = "πR²T + (2/3)π(R³ − ρ³) + (2/3)π(ρ³ − q³), R = 20, ρ = 18, T = 2, p = 3, q = √(ρ² − p²) = √315"
tex = '\pi R^2 T + \tfrac{2}{3}\pi\left(R^3 - \rho^3\right) + \tfrac{2}{3}\pi\left(\rho^3 - q^3\right),\quad R = 20,\ \rho = 18,\ T = 2,\ p = 3,\ q = \sqrt{\rho^2 - p^2} = \sqrt{315}'

[area]
value = 7352.665293339964
exact = "πR² + 2πRT + 2πR² + π(ρ² − p²) + 2πρ² − 2πρ(ρ − q) + 2πpq, with q as for the volume"
tex = '\pi R^2 + 2\pi RT + 2\pi R^2 + \pi\left(\rho^2 - p^2\right) + 2\pi\rho^2 - 2\pi\rho\,(\rho - q) + 2\pi p q'
+++

A closed hollow dome with a post inside it.

The dome: a plate, the disk of radius 20 about the z axis from z = −2 to z = 0, and on it a
hemispherical shell, everything above z = 0 between 18 and 20 from the origin. Inside them is a
closed cavity, the half ball of radius 18 about the origin above the plate.

The post: the disk of radius 3 about the z axis on the cavity's floor (the plate's top, z = 0),
extruded up (along +z) until it meets the inside of the dome, the sphere of radius 18, and joined
to the dome. So the post's side is a cylinder of radius 3 from z = 0 to z = √315 ≈ 17.748, and
its top is the cap of the sphere above the disk, rising to z = 18 on the axis. The cavity is
left as a ring about the post.
