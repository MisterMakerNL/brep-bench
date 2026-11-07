+++
title = "Two cylinders crossed at any angle, their common part, twenty ways"
tags = ["fuzz", "boolean", "common", "analytic", "ssi"]

[params]
r = [1, 10]
alpha = [30, 90]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[volume]
formula = "16*r**3/(3*sind(alpha))"
exact = "16r³/(3 sin α)"
tex = '\frac{16r^3}{3\sin\alpha}'

[area]
formula = "16*r**2/sind(alpha)"
exact = "16r²/sin α"
tex = '\frac{16r^2}{\sin\alpha}'
+++

Two cylinders of radius `r` and length 12`r`, each centered on the origin. The first lies along the
x axis, from x = −6`r` to 6`r`; the second along the line through the origin in the xy-plane at
angle `alpha` from the x axis, toward +y (its direction is (cos `alpha`, sin `alpha`, 0)). Make the
solid common to both: their surfaces meet in two ellipses, which cross each other at two points.

Then turn it by `psi` about the axis through the origin with direction (sin `theta` cos `phi`,
sin `theta` sin `phi`, cos `theta`), right-handed.

`boolean/steinmetz-common`, its cylinders crossed at any angle from 30° to square, in any
orientation.
