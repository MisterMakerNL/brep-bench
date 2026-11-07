+++
title = "A rod threaded by a square bar screwed along it"
tags = ["swept", "solid-sweep", "screw", "helix", "prism", "helicoid", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/square_thread.sv"

[volume]
value = 1300.5502131828668
exact = "π5²·20 − (20/6)·2π∫₄⁵ ρ w(ρ) dρ, w(ρ) = 2 + (6/π)·acos(4/ρ), with ∫ρ acos(a/ρ) dρ = ρ²acos(a/ρ)/2 − a√(ρ² − a²)/2"
tex = '\pi 5^2\cdot 20 - \tfrac{20}{6}\,2\pi\int_4^5 \rho\,w(\rho)\,d\rho,\quad w(\rho) = 2 + \tfrac{6}{\pi}\arccos\tfrac{4}{\rho}'

[area]
value = 821.16715556389149
exact = "the rod's side 2π·5·20·(1 − w(5)/6) + the root 2π·4·20·(2/6) + two ends 2(π5² − (2π/6)∫₄⁵ ρ w dρ) + two helicoidal walls 2(20/c)·½[√(U(U + c²)) + c² ln((√U + √(U + c²))/c)], c = 6/2π, U = 5² − 4² (each wall's element is ρ√(1 + c²/(ρ² − 4²)) dρ dθ)"
tex = '2\pi\cdot 5\cdot 20\left(1 - \tfrac{w(5)}{6}\right) + 2\pi\cdot 4\cdot 20\cdot\tfrac{2}{6} + 2\left(25\pi - \tfrac{2\pi}{6}\int_4^5 \rho w\,d\rho\right) + \tfrac{20}{c}\left[\sqrt{U(U + c^2)} + c^2\ln\tfrac{\sqrt U + \sqrt{U + c^2}}{c}\right],\ c = \tfrac{6}{2\pi},\ U = 9'
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a square thread cut
along it by a bar carried along a screw, as a lathe's tool cuts a thread.

The bar, as placed, is the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1: it points at the rod's axis, its
flat tip (the face x = 4) 1 inside the rod, and it is 2 wide along the axis.

The screw is about the z axis: the bar turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (the bar lowered 1.5, wholly below the
rod) to a turn of 1350° (raised 22.5, wholly above it). Cut from the rod everything the bar passes
through.

The groove's root is the cylinder of radius 4, where the tip passes. Its walls are where the tip's
two long edges pass, so the groove widens outward: at distance ρ from the axis it is 2 +
(6/π)·acos(4/ρ) wide along the axis, by the turn a point of an edge makes to come round to the tip's
plane. At the rod's surface that is 3.23 of every 6, so neighbouring turns of the groove stay apart.
