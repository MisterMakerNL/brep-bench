+++
title = "A rod threaded by a cone's point"
tags = ["swept", "solid-sweep", "screw", "helix", "cone", "apex", "boolean", "cut"]
origin = "geomsolver #721, box G5 (an apex on the tool's axis)"
settled = false
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a pointed cone
that points at the rod's axis, carried along a screw, as an engraving bit chases a thread.

The cone's axis is the x axis and its point is (4, 0, 0), 1 inside the rod. It opens away from the
rod at 60° (each side 30° from its axis) to a flat base at x = 12, of radius 8 tan 30°.

The screw is about the z axis: the cone turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (lowered 1.5, so that all of it within 5
of the z axis is below the rod) to a turn of 1350° (raised 22.5, all of that above it). Cut from the
rod everything the cone passes through.

The cone's point lies on its axis, and the groove's root is the helix it runs along: the contact on
each side runs down a root to the point and back up the other.
