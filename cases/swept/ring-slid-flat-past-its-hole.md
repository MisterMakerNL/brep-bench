+++
title = "A ring lying flat, slid across farther than its hole"
tags = ["swept", "solid-sweep", "slide", "torus", "flat-contact", "ring-in-plane"]
origin = "geomsolver #721, box D2 (a ring kept in a plane the motion keeps)"

[volume]
value = 1631.2657285967536
exact = "2πrR² + π²Rr² + 4πr³/3 + 2L(2rR + πr²/2), R = 6, r = 2, L = 15: each level is the slid annulus's outer stadium, since the slide passes its hole"
tex = '2\pi r R^2 + \pi^2 R r^2 + \tfrac{4}{3}\pi r^3 + 2L\left(2rR + \tfrac{\pi r^2}{2}\right),\quad R = 6,\ r = 2,\ L = 15'

[area]
value = 1061.826218357434
exact = "2(πR² + 2RL) (the flat top and bottom) + 2πr(πR + 2r) (the outer half of the torus, half at each end) + 2πrL (its outermost circle slid)"
tex = '2(\pi R^2 + 2RL) + 2\pi r(\pi R + 2r) + 2\pi r L'
+++

The solid a ring fills as it slides across, square to its own axis, farther than the width of its
hole.

The ring is a torus about the z axis centered on the origin: the circle through its tube's centers
has radius 6, and its tube has radius 2.

It slides without turning along the x axis by 15, ending centered on (15, 0, 0). Make the solid of
everything the ring passes through.

The torus's top and bottom circles, of radius 6 at z = ±2, stay in their planes as it slides, so
each sweeps a flat face: the region within 6 of the segment from (0, 0) to (15, 0). The slide is
longer than the hole is wide, so the hole is filled; the inner half of the torus touches nowhere.
