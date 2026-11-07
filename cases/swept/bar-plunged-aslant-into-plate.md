+++
title = "A bar plunged aslant into a plate, stopping inside"
tags = ["swept", "solid-sweep", "slide", "prism", "polyhedron", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/plunge.sv"

[volume]
value = 764.86960763082982
exact = "800 less the part within the plate of the convex hull of the bar where it starts and where it stops (a convex polyhedron, its measures exact: 35.130392369170131)"

[area]
value = 681.24504869617101
exact = "640 + that polyhedron's surface within the plate (74.502990500237274) − twice its face on z = 0 (16.628970902033146); every face a plane"
+++

A plate, the box from (0, −5, 0) to (20, 5, 4), with a pocket cut into its underside by a bar pushed
up aslant into it, stopping halfway through.

The bar is a prism of square section stood on a corner: everything with 6 ≤ x ≤ 14 and |y| + |z + 6|
≤ 1. Its section's corners are (y, z) = (0, −7), (1, −6), (0, −5) and (−1, −6).

It slides without turning by 8 along the direction (0.3, 0, √(1 − 0.3²)), leaning 0.3 toward +x from
straight up, so that it ends with its top edge at z = −5 + 8√0.91 ≈ 2.63, inside the plate. Cut from
the plate everything the bar passes through.

The bar is convex, so what it passes through is the convex hull of the bar where it starts and the
bar where it stops; the pocket is the part of that within the plate.
