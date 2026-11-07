+++
title = "A lens of two unequal balls, one placed aslant"
tags = ["boolean", "common", "analytic", "off-axis"]

[volume]
value = 42.245011348187795
exact = "πh₁²(3r₁ − h₁)/3 + πh₂²(3r₂ − h₂)/3, h₁ = r₁ − x, h₂ = r₂ − (d − x), x = (d² + r₁² − r₂²)/2d, r₁ = 5.511, r₂ = 3.317, d² = 4.231² + 3.467² + 2.653²"
tex = '\tfrac{\pi}{3}h_1^2(3r_1 - h_1) + \tfrac{\pi}{3}h_2^2(3r_2 - h_2),\quad h_1 = r_1 - x,\ h_2 = r_2 - (d - x),\ x = \frac{d^2 + r_1^2 - r_2^2}{2d},\ r_1 = 5.511,\ r_2 = 3.317,\ d^2 = 4.231^2 + 3.467^2 + 2.653^2'

[area]
value = 69.39098850747226
exact = "2πr₁h₁ + 2πr₂h₂"
tex = '2\pi r_1 h_1 + 2\pi r_2 h_2'
+++

Two balls: one of radius 5.511 centered on the origin, and one of radius 3.317 centered at
(4.231, 3.467, 2.653), about 6.08 from it, so that they overlap but neither holds the other. Make
the lens common to both.

`boolean/sphere-lens` with its balls unequal and the line between their centers off every axis, so
the circle where their spheres meet lies in a plane aslant to x, y and z.
