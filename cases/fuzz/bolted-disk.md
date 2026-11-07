+++
title = "A disk bored and drilled on a bolt circle, twenty ways"
tags = ["fuzz", "boolean", "cut", "pattern", "sequence"]

[family]
require = ["Rb - rh > rb + 0.02*Rd", "Rb + rh < 0.98*Rd", "2*Rb*sin(pi/n) > 2*rh + 0.02*Rd"]

[params]
Rd = [10, 40]
T = ["0.05*Rd", "0.5*Rd"]
rb = ["0.1*Rd", "0.4*Rd"]
n = { int = [3, 12] }
rh = ["0.02*Rd", "0.15*Rd"]
Rb = ["rb + rh", "Rd - rh"]
phi0 = [0, 360]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[let]
S = "Rd**2 - rb**2 - n*rh**2"

[volume]
formula = "pi*T*S"
exact = "πT(R_d² − r_b² − n r_h²)"
tex = '\pi T\left(R_d^2 - r_b^2 - n r_h^2\right)'

[area]
formula = "2*pi*S + 2*pi*T*(Rd + rb + n*rh)"
exact = "2π(R_d² − r_b² − n r_h²) + 2πT(R_d + r_b + n r_h)"
tex = '2\pi\left(R_d^2 - r_b^2 - n r_h^2\right) + 2\pi T\left(R_d + r_b + n r_h\right)'
+++

A disk of radius `Rd` and thickness `T`: the cylinder on the z axis from z = 0 to z = `T`. Bore a
hole of radius `rb` along its axis, and drill `n` holes of radius `rh` through it along z, their
centers evenly spaced on the circle of radius `Rb` about the axis, the first at angle `phi0` from
+x toward +y. No hole meets another or the disk's rim. If you cut the holes with cylinders, make
them run past both faces.

Then turn it by `psi` about the axis through the origin with direction (sin `theta` cos `phi`,
sin `theta` sin `phi`, cos `theta`), right-handed.

`boolean/drilled-plate` and `boolean/indexed-pattern`, of any count and proportions, in any
orientation.
