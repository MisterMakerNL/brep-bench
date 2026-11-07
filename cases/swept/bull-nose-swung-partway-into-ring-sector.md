+++
title = "A bull-nose mill swung partway into a curved block"
tags = ["swept", "solid-sweep", "turn", "torus", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/stopped_bullnose.sv"

[volume]
value = 71.991900957762336
exact = "80π/3 − ∫(π R(z)²/2 + (π/3)·10 R(z)) dz, R(z) the mill's radius at height z: 0.7 + √(0.3² − (z + 0.2)²) below z = −0.2, 1 above"

[area]
value = 175.62853108881793
exact = "40π + 16 − ∫2R dz (the opening at 0°) + the flat top and bottom swept, π R²/2 + (10π/3)R at R = 1 and 0.7 + the walls about the axis (π/6)·2·10·ℓ, ℓ = 0.7 + 0.3π/2 the side's profile + half the mill's side and round, π·0.7 + π·0.3(0.7π/2 + 0.3)"
+++

A curved block with a notch cut into it by a bull-nose mill swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The mill is a solid of revolution about the vertical line through (10, 0), from z = −0.5 to z = 0.5:
a cylinder of radius 1 whose bottom corner is rounded to radius 0.3. Its flat bottom is a disc of
radius 0.7 at z = −0.5, joined to its side (radius 1, from z = −0.2 up) by a quarter of a torus
whose tube, of radius 0.3, is centered on the circle of radius 0.7 at z = −0.2. Its top is flat, at
z = 0.5.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the mill passes
through.

The round's bottom circle stays in its plane, so the notch has a flat floor; the round's meridians
nearest and farthest from the axis sweep tori about the z axis.
