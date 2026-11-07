+++
title = "A coil spring pushed aslant through a plate"
tags = ["swept", "solid-sweep", "slide", "canal", "helix", "sweep-of-sweep", "free-form", "boolean", "cut"]
origin = "geomsolver #721, box G10 (sweeps of sweeps)"
settled = false
+++

A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched through it by a coil spring
pushed aslant wholly through it. The tool is itself a swept solid.

The spring is everything a ball of radius 0.4 passes through as its center runs along the helix of
radius 2 about the z axis that starts at (2, 0, −12), turns right-handed (from +x toward +y) and
rises 1.5 every turn, for three whole turns, ending at (2, 0, −7.5). Its coils are 1.5 apart and 0.8
thick, so they do not touch, and its ends are half balls.

It slides without turning along the vector (3, 1, 18), from wholly below the plate to wholly above
it. Cut from the plate everything the spring passes through.

Every level of the plate loses the spring's shadow along the push, a region bounded by curves the
coils' silhouettes draw, whose loops overlap one another.
