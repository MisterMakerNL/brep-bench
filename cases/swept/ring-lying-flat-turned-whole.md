+++
title = "A ring lying flat, turned a whole turn about a line beside it"
tags = ["swept", "solid-sweep", "turn", "whole-turn", "torus", "ring-in-plane", "pappus"]
origin = "geomsolver #721, box D2 (a ring kept in a plane the motion keeps) and D4 (a closed characteristic under a whole turn)"

[volume]
value = 761.09945990667004
exact = "2πD(4rR + πr²), D = 8, R = 3, r = 1 (Pappus: the shadow in a half-plane through the axis is everything within R + √(r² − z²) of ρ = D)"
tex = '2\pi D\left(4rR + \pi r^2\right),\quad D = 8,\ R = 3,\ r = 1'

[area]
value = 919.01313032409978
exact = "4πD(πr + 2R) (Pappus over the shadow's outline: two half circles and two flat lines 2R long)"
tex = '4\pi D\left(\pi r + 2R\right)'
+++

The solid a ring fills as it is turned a whole turn about an upright line beside it.

The ring is a torus whose axis is the vertical line through (8, 0): the circle through its tube's
centers has radius 3, and its tube has radius 1, centered on the plane z = 0.

It turns rigidly a whole turn about the z axis. Make the solid of everything the ring passes
through.

The ring's top and bottom circles stay in their planes and sweep flat annuli; its circles nearest
and farthest from the z axis at each height sweep the inner and outer walls. The result is a solid
ring about the z axis, from 4 to 12 out, its section in each half-plane through the z axis a
stadium: the band |z| ≤ 1 between 5 and 11 out, ended by half discs of radius 1.
