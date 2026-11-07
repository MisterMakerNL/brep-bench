+++
title = "A cylinder with its top rim rounded, twenty ways"
tags = ["fuzz", "fillet"]

[params]
R = [2, 20]
H = [2, 30]
rho = ["0.02*min(R, H)", "0.95*min(R, H)"]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[let]
d = "rho*(10 - 3*pi)/(12 - 3*pi)"

[volume]
formula = "pi*R**2*H - 2*pi*(R - d)*(1 - pi/4)*rho**2"
exact = "πR²H − 2π(R − d)(1 − π/4)ρ², d = ρ(10 − 3π)/(12 − 3π) (Pappus)"
tex = '\pi R^2 H - 2\pi(R - d)\left(1 - \tfrac{\pi}{4}\right)\rho^2,\quad d = \frac{\rho(10 - 3\pi)}{12 - 3\pi}'

[area]
formula = "pi*R**2 + 2*pi*R*(H - rho) + pi*(R - rho)**2 + pi**2*rho*(R - rho) + 2*pi*rho**2"
exact = "πR² + 2πR(H − ρ) + π(R − ρ)² + π²ρ(R − ρ) + 2πρ² (Pappus)"
tex = '\pi R^2 + 2\pi R(H - \rho) + \pi(R - \rho)^2 + \pi^2\rho(R - \rho) + 2\pi\rho^2'
+++

A cylinder of radius `R` on the z axis, from z = 0 to z = `H`. Round the circular edge around its
top face to radius `rho`.

Then turn it by `psi` about the axis through the origin with direction (sin `theta` cos `phi`,
sin `theta` sin `phi`, cos `theta`), right-handed.

`fillet/cylinder-rim`, of any proportions, its round from a hair to nearly all of the top.
