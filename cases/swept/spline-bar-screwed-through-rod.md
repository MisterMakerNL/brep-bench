+++
title = "A rod threaded by a bar whose section is a spline"
tags = ["swept", "solid-sweep", "screw", "helix", "bspline", "prism", "free-form", "boolean", "cut"]
origin = "geomsolver #721, box G10 (free-form faces)"
settled = false
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar whose
section is a closed B-spline curve, carried along a screw.

The bar's section lies in the plane y = 0: it is the closed, uniform, periodic cubic B-spline curve
(non-rational, knots evenly spaced, each span the cubic blend of four consecutive control points)
whose seven control points, in (x, z) and in order round the loop, are (3.5, 0), (4.5, −1.5), (8,
−1.5), (12, −1), (12, 1), (8, 1.5) and (4.5, 1.5). Its point nearest the rod's axis is (23/6, 0, 0),
about 1.17 inside the rod. The bar is everything within that curve carried from y = −8 to y = 8: a
prism, its sides one surface of extrusion of the spline.

The screw is about the z axis: the tool turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (lowered 1.5, the tool wholly below the
rod) to a turn of 1350° (raised 22.5, wholly above it). Cut from the rod everything the bar passes
through.

Its contact under the screw has no closed form on a spline: it must be found on the surface
numerically, at each ruling.
