+++
title = "A weighted barrel vault cut a hair below its crown"
tags = ["nurbs", "rational", "weights", "boolean", "cut", "shallow"]

[volume]
value = 8175.388634701551
exact = "the vault block 3168 + 5400·asin(4/5) less 24 times the circular segment δ = 1/1000 deep: 24(15²·acos(14.999/15) − 14.999·√(2·15·0.001 − 0.001²))"
tex = '3168 + 5400\arcsin\tfrac{4}{5} - 24\left(15^2\arccos\tfrac{15-\delta}{15} - (15-\delta)\sqrt{30\delta-\delta^2}\right),\ \delta = \tfrac{1}{1000}'

[area]
value = 2404.934758432357
exact = "1320 + 1170·asin(4/5), less the vault's strip 24·2·15·acos(14.999/15) and the two arched sides' segments, plus the cut's rectangle 24·2·√(2·15·0.001 − 0.001²)"
tex = '1320 + 1170\arcsin\tfrac{4}{5} - 24\cdot 30\arccos\tfrac{15-\delta}{15} + 48\sqrt{30\delta-\delta^2} - 2\,\mathrm{seg}(\delta),\ \delta = \tfrac{1}{1000}'
+++

The block of `nurbs/vault-tangent-plane` — the square −12 ≤ x, y ≤ 12, its floor at z = 0, its
sides upright, its top the rational B-spline surface of degree 1 along x and 2 along y whose
control points are (x, −12, 10), (x, 0, 26) and (x, 12, 10) for x = −12 and x = 12, weights 1, 3/5
and 1, which is the arch y² + (z − 1)² = 15² rising to its crown z = 16 along y = 0 — less
everything above the plane z = 16 − 1/1000.

That plane lies a thousandth below the crown. It crosses the vault along the two lines
y = ±√(2·15·0.001 − 0.001²) ≈ ±0.1732, z = 15.999, at under two thirds of a degree: the cut takes
a strip 0.35 wide off the top, its section a circular segment a thousandth deep. A kernel that
reads meetings by marching along the cross product of the two normals finds the angle too
shallow to follow; the case asks for the cut to be built.
