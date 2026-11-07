+++
title = "A rod threaded by a short bar whose corners cut"
tags = ["swept", "solid-sweep", "screw", "helix", "prism", "vertex", "helicoid", "boolean", "cut"]
origin = "geomsolver #721, box G1 (vertices)"

[volume]
value = 1326.8635603592684
exact = "π5²·20 − (20/6)·2π∫₄⁵ ρ w(ρ) dρ, w(ρ) = 2 + (6/π)·min(acos(4/ρ), asin(1.5/ρ)), the two meeting at ρ* = √(4² + 1.5²); ∫ρ acos(a/ρ) dρ = ρ²acos(a/ρ)/2 − a√(ρ² − a²)/2, ∫ρ asin(a/ρ) dρ = ρ²asin(a/ρ)/2 + a√(ρ² − a²)/2"

[area]
value = 882.29097252411702
exact = "the rod's side 2π·5·20·(1 − w(5)/6) + the root 2π·4·20·(2/6) + two ends 2(π5² − (2π/6)∫₄⁵ ρ w dρ) + two helicoidal walls 2(20/c)·[F(ρ*² − 4²) − F(0) + F(5² − 1.5²) − F(ρ*² − 1.5²)], F(u) = ½[√(u(u + c²)) + c² ln(√u + √(u + c²))], c = 6/2π"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut along it by
a short bar carried along a screw, so that the bar's corners, and not only its edges, carry the
groove's walls.

The bar, as placed, is the box 4 ≤ x ≤ 12, −1.5 ≤ y ≤ 1.5, −1 ≤ z ≤ 1: it points at the rod's axis,
its flat tip (the face x = 4) 1 inside the rod, 2 wide along the axis and 3 across.

The screw is about the z axis: the bar turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (the bar lowered 1.5, wholly below the
rod) to a turn of 1350° (raised 22.5, wholly above it). Cut from the rod everything the bar passes
through.

The bar's four corners (4, ±1.5, ±1) lie inside the rod. Out to distance √(4² + 1.5²) ≈ 4.272 from
the axis the groove's walls are carried by the tip's long edges, as a long bar's are; beyond it, by
the edges where the bar's sides y = ±1.5 meet its faces z = ±1. The two meet at the corners' paths.
