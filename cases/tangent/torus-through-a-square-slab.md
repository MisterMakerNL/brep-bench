+++
title = "A torus through a square slab, touching its four sides"
tags = ["tangent", "boolean", "union", "analytic", "torus", "planar", "point-contact", "ssi"]
origin = "jyviko/ssi-corpus c3_sphere_torus_outer"

[volume]
value = 2388.7516674059127
exact = "the slab 26²·2 + the torus 2π²Rr² less its part in the slab 4πR(c√(r² − c²) + r²·asin(c/r)), R = 10, r = 3, c = 1"
tex = '26^2\cdot 2 + 2\pi^2 R r^2 - 4\pi R\left(c\sqrt{r^2 - c^2} + r^2\arcsin\tfrac{c}{r}\right),\quad R = 10,\ r = 3,\ c = 1'

[area]
value = 1777.2602648670494
exact = "the slab 2·26² + 4·26·2 − two rings 2·4πR√(r² − c²) + the torus outside it 4π²Rr − 8πRr·asin(c/r), R = 10, r = 3, c = 1"
tex = '2\cdot 26^2 + 4\cdot 26\cdot 2 - 2\cdot 4\pi R\sqrt{r^2 - c^2} + 4\pi^2 R r - 8\pi R r\arcsin\tfrac{c}{r},\quad R = 10,\ r = 3,\ c = 1'
+++

A solid torus about the z axis, the middle circle of its tube of radius 10 in the xy plane and
the tube of radius 3, united with a square slab from (−13, −13, −1) to (13, 13, 1).

The torus reaches out to radius 13 only along its outer equator, the circle of radius 13 in the xy
plane, and that circle touches each of the slab's four sides at a single point, the middle of the
side: (13, 0, 0), (0, 13, 0), (−13, 0, 0) and (0, −13, 0). There the side and the torus are
tangent, and everywhere else near those points the torus lies inside the slab. The torus is
thicker than the slab: it comes out through the slab's top and bottom in the ring between radius
10 − √8 and 10 + √8. The union is the slab with half of the torus's tube standing out of each
face. The four points of contact are not vertices of the result: each side stays a whole
rectangle.

If the torus you make has a seam where its angle about z starts, turn it 45° about z, so that the
seam does not pass through a point of contact.
