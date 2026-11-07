+++
title = "A helical groove milled by a drum leaning to the screw's axis"
tags = ["swept", "solid-sweep", "screw", "helix", "cylinder", "leaning", "boolean", "cut"]
origin = "geomsolver #721, box X9 (a cylinder about an axis leaning to the screw's)"
settled = false
+++

A curved block with a helical groove milled through it by a drum leaning to the screw that carries
it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drum is a solid cylinder of radius 0.8 and length 3, its ends flat, whose axis runs through (10,
0, 0), its middle, along the direction (0, sin 25°, cos 25°): leaning 25° from upright toward +y. It
stands out above and below the block.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y) and
rises 1 for every whole turn, rigidly, from a turn of −140° to one of +140°. It starts clear of the
block, passes wholly through it, and ends clear of it. Cut from the block everything the drum passes
through.
