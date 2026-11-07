+++
title = "A small bar swung partway into a curved block"
tags = ["swept", "solid-sweep", "turn", "prism", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/stopped_turn.sv"

[volume]
value = 76.495376128929781
exact = "80π/3 − 1.2∫ρ(a(ρ) + π/6) dρ from 10 to √(11² + 0.5²), a(ρ) = asin(min(0.5, √(ρ² − 10²))/ρ) the bar's farthest angle from its middle plane at ρ; ∫ρ acos(a/ρ) and ∫ρ asin(a/ρ) in closed form"

[area]
value = 168.18615524888083
exact = "40π + 16 − 1.2(√121.25 − 10) (the opening at 0°) + floor and roof 2∫ρ(a + π/6) dρ + the walls 1.2(10π/6 + √121.25·(π/6 + asin(0.5/√121.25)) + 0.5 + 1), the last two the leading halves of the bar's inner face and its leading face"
+++

A curved block with a notch cut into it by a small bar swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is the box 10 ≤ x ≤ 11, −0.5 ≤ y ≤ 0.5, −0.6 ≤ z ≤ 0.6.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the bar passes
through.

The notch's far wall is the cylinder of radius √(11² + 0.5²) that the bar's outer corners sweep, its
near wall the cylinder of radius 10 its inner face sweeps, and it ends in the bar's own faces where
it stops.
