+++
title = "A napkin ring bored aslant, twenty ways"
tags = ["fuzz", "boolean", "cut", "analytic", "off-axis"]

[params]
R = [3, 15]
r = ["0.1*R", "0.9*R"]
theta = [0, 180]
phi = [0, 360]
cx = [-20, 20]
cy = [-20, 20]
cz = [-20, 20]

[let]
h = "2*sqrt(R**2 - r**2)"

[volume]
formula = "pi*h**3/6"
exact = "πh³/6, h = 2√(R² − r²)"
tex = '\tfrac{\pi h^3}{6},\quad h = 2\sqrt{R^2 - r^2}'

[area]
formula = "2*pi*R*h + 2*pi*r*h"
exact = "2πRh + 2πrh"
tex = '2\pi R h + 2\pi r h'
+++

A ball of radius `R` centered on (`cx`, `cy`, `cz`), with a hole of radius `r` bored right through
it along the line through its center with direction (sin `theta` cos `phi`, sin `theta` sin `phi`,
cos `theta`). If you cut the hole with a cylinder, make it run past the ball at both ends.

`boolean/napkin-ring`, of any size, bored in any direction, anywhere.
