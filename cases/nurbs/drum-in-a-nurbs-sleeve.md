+++
title = "A drum united with a sleeve whose bore is a NURBS cylinder"
tags = ["nurbs", "rational", "boolean", "union", "coincident", "cylinder"]
origin = "geomsolver #721, box X10 (a sheet lying in another kind of surface)"

[volume]
value = 1237.7875055143785
exact = "π5²·10 + π(7² − 5²)·6"
tex = '394\pi'

[area]
value = 697.4335690969341
exact = "150π + 192π − 2·(2π·5·6): the sleeve's bore lies on the drum's side"
tex = '222\pi'
+++

A drum and a sleeve around it, united. The sleeve's bore is the drum's own cylinder, but written as
a NURBS surface, so the union must recognise a B-spline face lying exactly on a cylinder.

The drum is the solid cylinder of radius 5 about the z axis from z = 0 to z = 10, its side a
circular cylinder.

The sleeve is everything between 5 and 7 from the z axis with 2 ≤ z ≤ 8. Its outer side is the
circular cylinder of radius 7 and its ends are flat annuli. Its bore, the side of radius 5, is a
rational B-spline surface of degree 2 around the axis and 1 along it: around the axis, the knot
vector (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1) and the nine control points, in (x, y), (0, 5), (−5, 5),
(−5, 0), (−5, −5), (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights 1, √2/2, 1, √2/2, 1, √2/2,
1, √2/2, 1; along the axis, the knot vector (0, 0, 1, 1) and the heights z = 2 and z = 8. So it is
exactly the cylinder of radius 5, its seam on the line through (0, 5).

The union is the drum with a ring 2 thick around its middle.
