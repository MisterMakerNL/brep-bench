+++
title = "A drum swung partway into a curved block"
tags = ["swept", "solid-sweep", "turn", "cylinder", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/stopped_drum.sv"

[volume]
value = 71.733032256966946
exact = "80π/3 − (π r²/2 + (π/6)·2Dr)·1, r = 1, D = 10: each circle about the axis loses the angle from the block's edge to 30° past it and the drum's half-width there, and ∫2ρ a(ρ) dρ is the drum's section, πr²"
tex = '\tfrac{137\pi}{6}'

[area]
value = 177.36281798666925
exact = "40π + 16 − 2 (the opening in the face at 0°) + floor and roof 2(π/2 + 10π/3) + the walls at radii 9 and 11 (π/6)(9 + 11)·1 + half the drum's side π = 52π + 14"
tex = '52\pi + 14'
+++

A curved block with a notch cut into it by a drum swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drum is a solid cylinder of radius 1 about the vertical line through (10, 0), from z = −0.5 to z
= 0.5, its ends flat.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the drum passes
through.

The notch runs in from the block's flat side at 0°, between the cylinders of radii 9 and 11 about
the z axis and the planes z = ±0.5, and ends in the half of the drum's side that leads where it
stops.
