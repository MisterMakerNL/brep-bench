+++
title = "A wheel carried along a spline, cutting with its rim"
tags = ["swept", "solid-sweep", "frame", "general-motion", "spline", "torus", "boolean", "cut"]
origin = "geomsolver #721, box X11 (K2 past convex tools of revolution)"
settled = false
+++

A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top by a ring carried
along a spline, standing up like a wheel across it.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (10, 0,
0), (20, 20, 0) and (30, 20, 0), run from its first end to its last.

A frame rides along the curve: at each point its origin is the curve's point, its x axis the curve's
tangent there (pointing the way the curve runs), its z axis straight up and its y axis to the left,
so that at the curve's start it is the world's own frame. The tool is drawn in that starting frame
and carried rigidly by the frame as it runs the whole curve.

The ring, so drawn, is a torus centered at (0, 0, 0.5) whose axis is the y axis direction: the
circle through its tube's centers has radius 1.5, and its tube has radius 0.5. So it stands upright
across the curve like a wheel, its lowest point 1.5 below the block's top.

Cut from the block everything the ring passes through as the frame runs the whole curve.

The ring is no convex solid: as the curve bends, its inner side comes into contact too.
