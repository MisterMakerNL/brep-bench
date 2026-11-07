+++
title = "A rod threaded by a wheel whose section is an ellipse"
tags = ["swept", "solid-sweep", "screw", "helix", "ellipsoid", "quadric", "boolean", "cut"]
origin = "geomsolver #721, box G8 (meridians that are no line or circle)"
settled = false
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove ground
around it by a wheel shaped as a flattened spheroid, carried along a screw. The wheel's meridian is
an ellipse, neither a line nor a circle.

The wheel is the oblate spheroid centered at (10.5, 0, 0) with semi-axes 6, 6 and 1.5, its short
axis along its own axis. That axis lies square to the x axis, leaning from the y axis down toward −z
by h = atan(2π·4.75/30) ≈ 44.83°: its direction is (0, cos h, −sin h). So its rim, a circle of
radius 6 about its center in the plane square to that axis, reaches to (4.5, 0, 0), 0.5 inside the
rod.

The screw is about the z axis: the wheel turns right-handed about it (from +x toward +y) and rises
30 for every whole turn, rigidly. It runs from a turn of −90° (lowered 7.5, wholly below the rod) to
a turn of +330° (raised 27.5, wholly above it). Cut from the rod everything the wheel passes
through.
