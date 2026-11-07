+++
title = "A NURBS cylinder with its top rim rounded"
tags = ["fillet", "nurbs", "rational", "torus"]

[volume]
value = 778.957433419712
exact = "πR²h − 2π(R − d)(1 − π/4)r², d = r(10 − 3π)/(12 − 3π) (Pappus), R = 5, h = 10, r = 1"
tex = '\pi R^2 h - 2\pi(R - d)\left(1 - \tfrac{\pi}{4}\right)r^2,\quad d = \frac{r(10 - 3\pi)}{12 - 3\pi},\ R = 5,\ h = 10,\ r = 1'

[area]
value = 457.31024053179993
exact = "150π − 9π − 10π + 2π(4 + 2/π)(π/2) (Pappus)"
tex = '150\pi - 9\pi - 10\pi + 2\pi\left(4 + \tfrac{2}{\pi}\right)\tfrac{\pi}{2}'
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 10, whose side is one rational B-spline
surface (a NURBS) and whose ends are flat disks on the planes z = 0 and z = 10.

The side is the tensor product of the standard rational quadratic circle and a straight line.
Around the axis (u), the circle of radius 5 of nine control points 5·cᵢ, cᵢ = (1, 0), (1, 1),
(0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1), (1, −1), (1, 0), with weights wᵢ = 1, √2/2, 1, √2/2,
1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1). Along the axis (v), degree
1 with two control points, z = 0 and z = 10, on the knots (0, 0, 1, 1). The net's control point
(i, j) is (5cᵢₓ, 5cᵢᵧ, zⱼ), with weight wᵢ. Its edges u = 0 and u = 1 are the same line, in the half
plane y = 0, x ≥ 0, which is the side's seam. Do not use a cylinder for the side.

Round the circular edge where the side meets the top to radius 1. The round is a blend between the
NURBS side and the flat top: the outer quarter of a torus's tube of radius 1 whose center circle,
of radius 4, lies in the plane z = 9, tangent to the side along z = 9 and to the top along the
circle of radius 4.

The result is the same solid as `fillet/cylinder-rim`, its side a NURBS.
