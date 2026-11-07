+++
title = "A slot swung by a pin lying along a radius"
tags = ["swept", "solid-sweep", "turn", "revolution", "sphere", "ring-contact", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/spoke.sv"

[volume]
value = 53.843407424025067
exact = "(π/3)·80 − (π/6)(57.25 − 1/12): the slot is |z| ≤ 0.5 out from ρ = 8 to the sphere ρ² + z² = 11² + 0.5², over the whole 60°"
tex = '\tfrac{80\pi}{3} - \tfrac{\pi}{6}\left(57.25 - \tfrac{1}{12}\right)'

[area]
value = 198.4923077874897
exact = "the block 40π + 16, less the inner wall's band 8π/3 and two openings ∫(√(121.25 − z²) − 8) dz over |z| ≤ 0.5, plus roof and floor (π/6)(11² − 8²) each and a sixth of the sphere's zone 2π√121.25·1"
tex = '40\pi + 16 - \tfrac{8\pi}{3} - 2\int_{-1/2}^{1/2}\left(\sqrt{121.25 - z^2} - 8\right)dz + \tfrac{\pi}{3}(121 - 64) + \tfrac{2\pi\sqrt{121.25}}{6}'
+++

A curved block with a slot swung through it by a pin.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The pin is a solid cylinder of radius 0.5 whose axis is the x axis, from x = 6 to x = 11, its ends
flat. Its axis meets the block's at the origin.

The pin turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it
starts clear of the block, passes through it, and ends clear of it. Cut from the block everything
the pin passes through.

Each circle of the pin about its own axis touches at its two points in the plane of the two axes, so
the pin's side sweeps the slot's flat roof and floor, z = ±0.5. On the rim of the pin's far end the
circle whose normals pass through the origin touches all round and sweeps a zone of the sphere of
radius √(11² + 0.5²) about the origin, the slot's end wall. The slot runs right through the block's
inner wall and both its flat sides, so the block falls into what lies above the slot and what lies
below it, joined beyond the sphere.
