+++
title = "A cube tumbled through space"
tags = ["swept", "solid-sweep", "tumble", "general-motion", "polyhedron"]
origin = "geomsolver #721, box K3 (nested relative motions)"
settled = false
+++

The solid a cube fills as it tumbles: it turns about one axis while being watched from a frame that
turns about another.

The cube is the box from (2.4, −0.6, 1.4) to (3.6, 0.6, 2.6): edge 1.2, centered at (3, 0, 2).

It moves by two turns at once, both driven by one parameter t:

- it turns by t/4 about the vertical line through (2, 0, 0), right-handed (from +x toward +y);
- the frame it is seen from turns by t about the line through (0, 0.5, 0.5) parallel to the x axis,
  right-handed (from +y toward +z).

Seen from that frame, the cube's place at t is its place turned by t/4 about the first line and then
by −t about the second. Make the solid of everything the cube passes through, so seen, as t runs
from −75° to 75°.

No turn keeps any line of the cube fixed, and its silhouette changes from face to face as it
tumbles.
