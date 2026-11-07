+++
title = "A rod threaded by an end mill's flat end"
tags = ["swept", "solid-sweep", "screw", "helix", "cylinder", "cap", "rim", "boolean", "cut"]
origin = "geomsolver #721, box G3 (a cap's round rim under a twist)"
settled = false
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by the flat end of
an end mill that points at the rod's axis, carried along a screw.

The mill is a solid cylinder of radius 1.2 whose axis is the x axis, from x = 4 to x = 12: its flat
end, the disc of radius 1.2 in the plane x = 4, is 1 inside the rod.

The screw is about the z axis: the tool turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (lowered 1.5, the tool wholly below the
rod) to a turn of 1350° (raised 22.5, wholly above it). Cut from the rod everything the mill passes
through.

The flat end touches along a line across it and its round rim, a sharp circle, carries the contact
round to the side: the groove's floor is cut by the end and its walls by the rim and the side.
