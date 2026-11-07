+++
title = "A half drum swung partway into a curved block, its arcs ending at its cut"
tags = ["swept", "solid-sweep", "turn", "cylinder", "stopped", "boolean", "cut"]
origin = "geomsolver #721, box X7 (the half drum turned part of a turn)"

[volume]
value = 73.303828583761842
exact = "80π/3 − (π/6)(11² − 9²)/2·1: the half drum leads with its flat cut, so each circle about the axis between 9 and 11 loses exactly the sector's first 30°"
tex = '\tfrac{80\pi}{3} - \tfrac{10\pi}{3}'

[area]
value = 173.07963267948966
exact = "40π + 16 − 2 (the opening at 0°) + floor and roof 2·10π/3 + the walls (π/6)(9 + 11) + the half drum's flat face 2·1 where it stops"
tex = '50\pi + 16'
+++

A curved block with a notch cut into it by half a drum swung partway into it, flat face first.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The tool is half of the solid cylinder of radius 1 about the vertical line through (10, 0), from z =
−0.5 to z = 0.5: the half with y ≤ 0, cut off by the plane y = 0 through its axis. Its flat face is
the rectangle 9 ≤ x ≤ 11, y = 0, |z| ≤ 0.5, and its round side faces −y.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the half drum passes
through.

The half drum leads with its flat face, so the notch is the ring between radii 9 and 11 and heights
±0.5 from the block's edge at 0° to the flat face where it stops, at 30°. Its round side's arcs end
at that face, at the two corners whose paths are the notch's inner and outer edges.
