+++
title = "A slot punched by a drum pushed edge first"
tags = ["swept", "solid-sweep", "slide", "cylinder", "closed-characteristic", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/punch.sv"

[volume]
value = 672.0
exact = "800 − 8·(2·2)·4"
tex = '672'

[area]
value = 672.0
exact = "640 − 2·32 + (2·8 + 2·4)·4"
tex = '672'
+++

A plate, the box from (0, −5, 0) to (20, 5, 4), with a slot punched through it by a drum.

The drum is a solid cylinder of radius 2 lying on its side: its axis is the line y = 0, z = −5, and
it runs from x = 6 to x = 14, its ends flat.

It slides without turning straight up by 16, from wholly below the plate to wholly above it. Cut
from the plate everything the drum passes through.

The drum touches what it sweeps along a closed curve: along its side's lowest and highest lines
nowhere (they move along themselves), along the two lines of its side at y = ±2, and across both
flat ends, which the push keeps in their own planes. The slot is its shadow along the push, the
rectangle 6 ≤ x ≤ 14, |y| ≤ 2, through the plate.
