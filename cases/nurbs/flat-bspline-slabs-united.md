+++
title = "Two slabs united along flat B-spline faces in one plane"
tags = ["nurbs", "bspline", "boolean", "union", "coincident", "coplanar"]
origin = "geomsolver #721, box X10 (two sheets lying in one plane)"

[volume]
value = 700.0
exact = "10·10·4 + 10·10·3"
tex = '700'

[area]
value = 610.0
exact = "360 + 320 − 2·(5·7): the two flat faces share 5 × 7 of the plane z = 4"
tex = '610'
+++

Two slabs united, touching along the plane z = 4, where each has a face that is a B-spline patch
lying flat in that plane. The two patches are parametrised differently, and they overlap in part.

The first slab is the box from (0, 0, 0) to (10, 10, 4). Its top is a non-rational bicubic Bézier
patch (degree 3 both ways, knot vectors (0, 0, 0, 0, 1, 1, 1, 1)) whose 4 × 4 control points are
(xᵢ, yⱼ, 4) with x₀ … x₃ = 0, 2, 7, 10 and y₀ … y₃ = 0, 4, 5, 10. Its other five faces are planes.

The second slab is the box from (5, 3, 4) to (15, 13, 7). Its bottom is a non-rational biquadratic
Bézier patch (degree 2 both ways, knot vectors (0, 0, 0, 1, 1, 1)) whose 3 × 3 control points are
(xᵢ, yⱼ, 4) with x₀ … x₂ = 5, 12, 15 and y₀ … y₂ = 3, 4, 13. Its other five faces are planes.

Every control point of each patch lies in the plane z = 4, and each patch's x and y rise steadily
with its parameters, so each patch is exactly its square of that plane. The union is one solid; the
two patches overlap on the rectangle 5 ≤ x ≤ 10, 3 ≤ y ≤ 10.
