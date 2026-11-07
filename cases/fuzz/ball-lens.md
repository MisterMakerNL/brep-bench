+++
title = "A lens of two unequal balls, twenty ways"
tags = ["fuzz", "boolean", "common", "analytic"]

[params]
r1 = [2, 10]
r2 = [2, 10]
d = ["abs(r1 - r2) + 0.05*min(r1, r2)", "r1 + r2 - 0.05*min(r1, r2)"]
theta = [0, 180]
phi = [0, 360]

[let]
x = "(d**2 + r1**2 - r2**2)/(2*d)"
h1 = "r1 - x"
h2 = "r2 - (d - x)"

[volume]
formula = "pi*h1**2*(3*r1 - h1)/3 + pi*h2**2*(3*r2 - h2)/3"
exact = "πh₁²(3r₁ − h₁)/3 + πh₂²(3r₂ − h₂)/3, h₁ = r₁ − x, h₂ = r₂ − (d − x), x = (d² + r₁² − r₂²)/2d"
tex = '\tfrac{\pi}{3}h_1^2(3r_1 - h_1) + \tfrac{\pi}{3}h_2^2(3r_2 - h_2),\quad h_1 = r_1 - x,\ h_2 = r_2 - (d - x),\ x = \frac{d^2 + r_1^2 - r_2^2}{2d}'

[area]
formula = "2*pi*r1*h1 + 2*pi*r2*h2"
exact = "2πr₁h₁ + 2πr₂h₂"
tex = '2\pi r_1 h_1 + 2\pi r_2 h_2'
+++

Two balls: one of radius `r1` centered on the origin, and one of radius `r2` centered at distance
`d` from it, at `d` (sin `theta` cos `phi`, sin `theta` sin `phi`, cos `theta`). Neither holds the
other, and they overlap. Make the lens common to both.

`boolean/sphere-lens`, its balls unequal, from nearly one inside the other to nearly apart.
