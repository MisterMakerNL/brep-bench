+++
title = "A torus sliced across its axis, twenty ways"
tags = ["fuzz", "boolean", "cut", "analytic", "torus"]

[params]
R = [3, 20]
r = ["0.1*R", "0.9*R"]
h = ["-0.9*r", "0.9*r"]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[let]
w = "sqrt(r**2 - h**2)"
S = "r**2*acos(h/r) - h*w"

[volume]
formula = "2*pi*R*S"
exact = "2πR·S, S = r² arccos(h/r) − h√(r² − h²) (Pappus: the segment of the tube's circle above z = h)"
tex = '2\pi R S,\quad S = r^2\arccos\frac{h}{r} - h\sqrt{r^2 - h^2}'

[area]
formula = "4*pi*R*r*acos(h/r) + 4*pi*R*w"
exact = "4πRr arccos(h/r) + 4πR√(r² − h²) (Pappus, and the annulus cut)"
tex = '4\pi R r\arccos\frac{h}{r} + 4\pi R\sqrt{r^2 - h^2}'
+++

A solid torus centered on the origin, about the z axis: the circle through the middle of its tube
has radius `R`, and the tube has radius `r`. Cut it by the plane z = `h` and keep the part above
it, where z ≥ `h`.

Then turn it by `psi` about the axis through the origin with direction (sin `theta` cos `phi`,
sin `theta` sin `phi`, cos `theta`), right-handed.

`boolean/half-torus`, of any proportions, sliced at any height through its tube.
