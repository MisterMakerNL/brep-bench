+++
title = "A drum plunged aslant into a plate, stopping inside"
tags = ["swept", "solid-sweep", "slide", "cylinder", "oblique", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/stopped_punch.sv"
settled = false
+++

A plate, the box from (0, −5, 0) to (20, 5, 4), with a pocket cut into its underside by a drum
pushed up aslant into it, stopping inside.

The drum is a solid cylinder of radius 2 lying on its side: its axis is the line y = 0, z = −5, and
it runs from x = 6 to x = 14, its ends flat.

It slides without turning by 8 along the direction (0.3, 0, √(1 − 0.3²)), leaning 0.3 toward +x from
straight up, so that its axis ends at height −5 + 8√0.91 ≈ 2.63, inside the plate. Cut from the
plate everything the drum passes through.

Pushed aslant, the drum's flat ends lean to the push: each rim sweeps an oblique elliptic cylinder,
the side's lines at y = ±2 sweep the planes tangent along them, and the pocket ends in the drum's
own side where it stops.
