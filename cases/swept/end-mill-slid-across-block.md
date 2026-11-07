+++
title = "A slot cut by an end mill slid across a block"
tags = ["swept", "solid-sweep", "slide", "cylinder", "flat-contact", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/slot.sv"

[volume]
value = 1760.0
exact = "20·10·10 − 20·(2·2)·3"
tex = '1760'

[area]
value = 1096.0
exact = "1000 − 80 − 2·12 + 80 + 2·20·3"
tex = '1096'
+++

A block, the box from (0, −5, 0) to (20, 5, 10), with a slot cut across its top by an end mill.

The end mill is a solid cylinder of radius 2 standing upright about the vertical line through (−5,
0), from z = 7 up to z = 15: its flat end is 3 below the block's top.

It slides without turning along the x axis by 30, from x = −5 to x = 25, from clear before the block
to clear past it. Cut from the block everything the mill passes through.

Its flat end, which the slide keeps in its own plane, touches all over and sweeps the slot's floor;
the two lines of its side square to the slide sweep the walls, meeting the floor in sharp edges. The
slot is |y| ≤ 2, z ≥ 7, along the block's whole length.
