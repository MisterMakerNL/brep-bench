+++
title = "A bull-nose mill pushed straight down into a block at a lean"
tags = ["swept", "solid-sweep", "slide", "torus", "cylinder", "leaning", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/leaning_bullnose.sv"
settled = false
+++

A block, the box from (−8, −8, −8) to (8, 8, 0), with a hole bored into its top by a bull-nose mill
pushed straight down into it at a lean, stopping wholly inside it.

The mill is a solid of revolution about an axis leaning 30° from upright toward +x, along the
direction a = (sin 30°, 0, cos 30°), through the point c = (0, 0, 4), the middle of its flat end.
Along its axis it runs from c up to c + 3a. It is a cylinder of radius 2 whose end corner is rounded
to radius 0.5: its flat end is a disc of radius 1.5 about c, square to a; its side, of radius 2,
runs from c + 0.5a to c + 3a, where it is flat; and the round between them is a quarter of a torus
whose tube, of radius 0.5, is centered on the circle of radius 1.5 about c + 0.5a.

It slides without turning straight down by 8, so that its end's middle ends at (0, 0, −4), the whole
mill under the block's top. Cut from the block everything the mill passes through.
