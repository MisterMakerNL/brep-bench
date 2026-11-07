+++
title = "A bull-nose mill run along a line, an arc and a line"
tags = ["swept", "solid-sweep", "path", "slide", "turn", "torus", "pieces", "boolean", "cut"]
origin = "geomsolver #721, box K1 (paths made of pieces)"

[volume]
value = 24653.516723445
exact = "50·50·10 − (A·L + V₀), L = 40 + 5π the path's length, A = ∫2R(z) dz the groove's section below the top, V₀ = ∫πR(z)² dz the mill below the top, R(z) = 1.5 + √(0.25 − (z + 1)²) below z = −1, 2 above (Pappus on the arc: the section is symmetric about the path)"

[area]
value = 7159.2554104406572
exact = "7000 − (4L + 4π) (the opening) + (3L + 2.25π) (the floor) + 2L(0.5π/2 + 1) (the walls) + the mill's side and round below the top, 2π·0.5(1.5π/2 + 0.5) + 2π·2·1"
+++

A block, the box from (−10, −10, −10) to (40, 40, 0), with a groove cut into its top by a bull-nose
mill run along a path made of a line, an arc and a line.

The mill is a solid of revolution about an upright axis: a cylinder of radius 2 from 1.5 below the
block's top up to 5 above it, whose bottom corner is rounded to radius 0.5. Its flat bottom is a
disc of radius 1.5 at z = −1.5, joined to its side (radius 2, from z = −1 up) by a quarter of a
torus whose tube, of radius 0.5, is centered on the circle of radius 1.5 at z = −1.

Its axis starts on the vertical line through (0, 0) and moves, staying upright, in three pieces:

- it slides along the x axis to (20, 0);
- it turns a quarter turn about the vertical line through (20, 10), right-handed (from −y toward
  +x), carried rigidly, to (30, 10);
- it slides along +y to (30, 30).

Cut from the block everything the mill passes through.

The path is tangent where its pieces meet, and its arc, of radius 10, is wider than the mill. The
round's bottom circle stays in its plane, so the groove has a flat floor 3 wide.
