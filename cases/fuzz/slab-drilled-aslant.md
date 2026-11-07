+++
title = "A slab drilled through aslant, twenty ways"
tags = ["fuzz", "boolean", "cut", "analytic", "off-axis"]

[family]
require = ["T/2*tand(beta) + r/cosd(beta) < 0.45*min(W, D)"]

[params]
W = [10, 40]
D = [10, 40]
T = [2, 15]
r = ["0.03*min(W, D)", "0.2*min(W, D)"]
beta = [0, 60]
phi = [0, 360]

[volume]
formula = "W*D*T - pi*r**2*T/cosd(beta)"
exact = "WDT − πr²T/cos β"
tex = 'WDT - \frac{\pi r^2 T}{\cos\beta}'

[area]
formula = "2*W*D + 2*T*(W + D) - 2*pi*r**2/cosd(beta) + 2*pi*r*T/cosd(beta)"
exact = "2WD + 2T(W + D) − 2πr²/cos β + 2πrT/cos β"
tex = '2WD + 2T(W + D) - \frac{2\pi r^2}{\cos\beta} + \frac{2\pi r T}{\cos\beta}'
+++

A slab from (−`W`/2, −`D`/2, 0) to (`W`/2, `D`/2, `T`). Drill a round hole of radius `r` right
through it, its axis the line through (0, 0, `T`/2) with direction (sin `beta` cos `phi`,
sin `beta` sin `phi`, cos `beta`): tilted `beta` from the vertical, toward the azimuth `phi`. The
hole leaves the top and bottom faces in ellipses and never breaks out of the slab's sides. If you
cut it with a cylinder, make it run past both faces.

`boolean/box-minus-through-cylinder`, of any proportions, drilled at any slant.
