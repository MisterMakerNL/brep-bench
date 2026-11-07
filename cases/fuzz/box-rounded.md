+++
title = "A box with every edge rounded, twenty ways"
tags = ["fuzz", "fillet", "corner"]

[params]
a = [4, 30]
b = [4, 30]
c = [4, 30]
rho = ["0.02*min(a, b, c)", "0.48*min(a, b, c)"]
theta = [0, 180]
phi = [0, 360]
psi = [0, 360]

[let]
p = "a - 2*rho"
q = "b - 2*rho"
s = "c - 2*rho"

[volume]
formula = "p*q*s + 2*(p*q + q*s + s*p)*rho + pi*(p + q + s)*rho**2 + 4*pi*rho**3/3"
exact = "pqs + 2(pq + qs + sp)ρ + π(p + q + s)ρ² + 4πρ³/3, p = a − 2ρ, q = b − 2ρ, s = c − 2ρ"
tex = 'pqs + 2(pq + qs + sp)\rho + \pi(p + q + s)\rho^2 + \tfrac{4}{3}\pi\rho^3,\quad p = a - 2\rho,\ q = b - 2\rho,\ s = c - 2\rho'

[area]
formula = "2*(p*q + q*s + s*p) + 2*pi*(p + q + s)*rho + 4*pi*rho**2"
exact = "2(pq + qs + sp) + 2π(p + q + s)ρ + 4πρ²"
tex = '2(pq + qs + sp) + 2\pi(p + q + s)\rho + 4\pi\rho^2'
+++

A box from the origin to (`a`, `b`, `c`) with all twelve of its edges rounded to radius `rho`.
Three rounds meet at each corner, and the corner is the eighth of a ball of radius `rho` that
they run into: the solid is every point within `rho` of the box from (`rho`, `rho`, `rho`) to
(`a` − `rho`, `b` − `rho`, `c` − `rho`).

Then turn it by `psi` about the axis through the origin with direction (sin `theta` cos `phi`,
sin `theta` sin `phi`, cos `theta`), right-handed.

`fillet/box-all-edges`, of any proportions, its rounds from a hair to nearly half its least side.
