+++
title = "A pointed drill swung partway into a curved block"
tags = ["swept", "solid-sweep", "turn", "cone", "sharp-edge", "stopped", "boolean", "cut"]
origin = "geomsolver #721, box X7 (a sharp rim beside a cone, turned)"

[volume]
value = 65.874133399095044
exact = "80π/3 − ∫(π R(z)²/2 + (π/3)·10 R(z)) dz over −0.8 ≤ z ≤ 1, R(z) = min((z + 0.8)·tan 59°, 1)"

[area]
value = 156.99586044887178
exact = "40π + 16 − ∫2R dz (the opening at 0°) − (π/2 + 10π/3) (the opening in the top) + the walls about the axis (π/6)·2·10·ℓ, ℓ = 1/sin 59° + 1.8 − 1/tan 59° the profile's length + half the drill's surface below z = 1, (π/sin 59° + 2π(1.8 − 1/tan 59°))/2"
+++

A curved block with a notch cut down into it by a drill swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drill is a solid of revolution about the vertical line through (10, 0): a cylinder of radius 1
from z = −0.8 + 1/tan 59° up to z = 3, ended below in a cone of 118° (each side 59° from the axis)
whose point is (10, 0, −0.8). The point is in the block; the drill stands out through its top.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the drill passes
through.

The rim where the cone meets the cylinder is a sharp circle. The turn carries the cone's side and
the cylinder's into cones and cylinders about the z axis, which meet along the circles the rim's
nearest and farthest points sweep.
