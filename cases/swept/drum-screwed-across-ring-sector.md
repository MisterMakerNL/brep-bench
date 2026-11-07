+++
title = "A helical groove milled by a drum screwed across a block"
tags = ["swept", "solid-sweep", "screw", "helix", "cylinder", "helicoid", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/helical_groove.sv"
settled = false
+++

A curved block with a helical groove milled through it by a drum screwed across it, as an end mill
mills a helical groove.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drum is a solid cylinder of radius 1 about the vertical line through (10, 0), from z = −0.5 to z
= 0.5, its ends flat.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y) and
rises 0.5 for every whole turn, rigidly, from a turn of −140° to one of +140°. It starts clear of
the block, passes wholly through it, and ends clear of it. Cut from the block everything the drum
passes through.

The drum's flat ends keep no plane: each sweeps a thin helical ramp.
