+++
title = "A domed mill swung partway into a curved block"
tags = ["swept", "solid-sweep", "turn", "sphere", "sharp-edge", "stopped", "boolean", "cut"]
origin = "geomsolver #721, box X7 (a sharp rim beside a ball, turned)"

[volume]
value = 63.212952213117267
exact = "80π/3 − ∫(π R(z)²/2 + (π/3)·10 R(z)) dz over −0.8 ≤ z ≤ 1, R(z) = min(√(4 − (z − 1.2)²), 1)"

[area]
value = 159.70096803807784
exact = "40π + 16 − ∫2R dz (the opening at 0°) − (π/2 + 10π/3) (the opening in the top) + the walls about the axis (π/6)·2·10·ℓ, ℓ = π/3 + √3 − 0.2 the profile's length + half the mill's surface below z = 1, (4π(2 − √3) + 2π(√3 − 0.2))/2"
+++

A curved block with a notch cut down into it by a mill with a domed end swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The mill is a solid of revolution about the vertical line through (10, 0): a cylinder of radius 1
standing up to z = 3, ended below by a cap of the sphere of radius 2 centered at (10, 0, 1.2). The
cap's lowest point is (10, 0, −0.8), in the block, and it meets the cylinder in a sharp circle at z
= 1.2 − √3 ≈ −0.532, where the sphere's radius 2 and the cylinder's 1 cross at 30°.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the mill passes
through.

The turn carries the cap into tori about the z axis (each meridian of the cap an arc of a circle of
radius 2 off the axis) and the side into cylinders, meeting along the circles the sharp rim's
nearest and farthest points sweep.
