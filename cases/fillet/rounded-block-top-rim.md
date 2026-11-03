+++
title = "A rounded block with its top rim rounded"
tags = ["fillet", "round", "planar", "cylinder", "tangent-chain"]

[volume]
value = 1149.043632182972
exact = "AH − (1 − π/4)r²L − 2π(ρ − d)(1 − π/4)r² (Pappus), A = ab − (4 − π)ρ², L = 2(a − 2ρ) + 2(b − 2ρ), d = r(10 − 3π)/(12 − 3π), a = 20, b = 12, ρ = 3, H = 5, r = 1"
tex = 'AH - \left(1 - \tfrac{\pi}{4}\right)r^2 L - 2\pi(\rho - d)\left(1 - \tfrac{\pi}{4}\right)r^2,\quad A = ab - (4 - \pi)\rho^2,\ L = 2(a - 2\rho) + 2(b - 2\rho),\ d = \frac{r(10 - 3\pi)}{12 - 3\pi}'

[area]
value = 733.0931753639765
exact = "A + T + (L + 2πρ)(H − r) + (πr/2)L + 2π(ρ − r + 2r/π)(πr/2) (Pappus), T = (a − 2r)(b − 2r) − (4 − π)(ρ − r)² the top"
tex = 'A + T + (L + 2\pi\rho)(H - r) + \tfrac{\pi r}{2}L + 2\pi\left(\rho - r + \tfrac{2r}{\pi}\right)\tfrac{\pi r}{2},\quad T = (a - 2r)(b - 2r) - (4 - \pi)(\rho - r)^2'
+++

A block 20 by 12 by 5, from (−10, −6, 0) to (10, 6, 5), with its four vertical edges rounded to
radius 3: the rectangle with its corners rounded to radius 3, extruded 5 up the z axis. Round the
rim of its top face to radius 1.

The rim is eight edges, four straight and four quarter circles, each running on tangent into the
next, and the round runs round all eight unbroken: along the straight edges it is a quarter of a
cylinder of radius 1, and round the corners a quarter of the section of a torus, whose tube runs
round the corner's axis at radius 2. Nowhere do three edges meet, and nowhere does the round end.
