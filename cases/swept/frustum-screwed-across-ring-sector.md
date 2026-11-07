+++
title = "A helical groove milled by a tapered cutter"
tags = ["swept", "solid-sweep", "screw", "helix", "cone", "boolean", "cut"]
origin = "geomsolver #721, box X9 (a screwed cone cut from a blank)"
settled = false
+++

A curved block with a helical groove milled through it by a tapered cutter screwed across it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The cutter is a frustum of a cone about the vertical line through (10, 0): radius 0.4 at z = −0.6,
widening to radius 1 at z = 1.4, its ends flat. It stands out through the block's top.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y) and
rises 2 for every whole turn, rigidly, from a turn of −140° to one of +140°. It starts clear of the
block, passes wholly through it, and ends clear of it. Cut from the block everything the cutter
passes through.
