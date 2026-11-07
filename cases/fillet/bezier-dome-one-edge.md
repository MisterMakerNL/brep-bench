+++
title = "The Bézier dome block with one top edge rounded"
tags = ["fillet", "nurbs", "bezier", "surface"]
settled = false
+++

The block under a bicubic Bézier dome of `nurbs/bezier-dome`: standing on the square
0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0, its four sides upright in the
planes x = 0, x = 20, y = 0 and y = 20, and its top the bicubic Bézier patch whose sixteen control
points are Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ), i, j = 0, 1, 2, 3, at these heights zᵢⱼ (row i,
x = 20i/3; column j, y = 20j/3):

| | j = 0 | j = 1 | j = 2 | j = 3 |
|---|---|---|---|---|
| i = 0 | 4 | 6 | 7 | 5 |
| i = 1 | 6 | 14 | 12 | 7 |
| i = 2 | 5 | 13 | 16 | 8 |
| i = 3 | 3 | 6 | 8 | 6 |

The patch is S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ for u, v from 0 to 1, with the cubic Bernstein
polynomials B₀(t) = (1 − t)³, B₁(t) = 3t(1 − t)², B₂(t) = 3t²(1 − t), B₃(t) = t³, so that x = 20u
and y = 20v.

Round the edge where the patch meets the side y = 0, the cubic curve the patch's edge v = 0 draws
in that plane, to radius 1. The round is the blend a ball of radius 1 sweeps as it rolls along the
edge touching both the freeform top and the flat side; the angle between them changes along the
edge, and so does the round's section. At its ends the round runs into the sides x = 0 and
x = 20, whose planes trim it.

The round has no closed form, so the case is unsettled until two kernels agree.
