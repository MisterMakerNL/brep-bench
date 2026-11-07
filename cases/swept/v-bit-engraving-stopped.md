+++
title = "A V bit engraving a line that stops inside a block"
tags = ["swept", "solid-sweep", "slide", "cone", "apex", "stopped", "boolean", "cut"]
origin = "geomsolver #721, box G5 (an apex on the tool's axis)"

[volume]
value = 5931.6224195904272
exact = "6000 − (15·2² + (π/3)·2³): the groove's section, the triangle 4 wide and 2 deep, along 15, and the cone below the top at its two ends together"
tex = '6000 - \left(15\cdot 2^2 + \tfrac{\pi}{3}2^3\right)'

[area]
value = 2230.05797488066
exact = "2200 − (4π + 60) (the opening) + two flanks 2·15·2√2 + the cone's side below the top, half at each end, π·2·2√2"
tex = '2200 - (4\pi + 60) + 60\sqrt 2 + 4\sqrt 2\,\pi'
+++

A block, the box from (0, −10, −10) to (30, 10, 0), with a line engraved into its top by a V bit
that stops inside it at both ends.

The bit is a solid cone whose axis is the vertical line through (5, 0), its point at (5, 0, −2), 2
below the block's top, opening upward at 90° (each side 45° from the axis) to a flat top of radius 5
at z = 3.

It slides without turning along the x axis by 15, so that its point ends at (20, 0, −2). Cut from
the block everything the bit passes through.

The bit's point lies on its axis, inside the block at both ends. The groove is a V 4 wide at the top
and 2 deep, along 15, its flanks planes at 45°, ended at each end by half of the cone.
