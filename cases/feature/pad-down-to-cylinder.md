+++
title = "A square pad extruded down to a cylinder"
tags = ["feature", "extrude", "up-to-face", "cylinder", "union", "analytic"]

[volume]
value = 12751.845965694885
exact = "πR²L + 2a(2ah − S), S = a√(R² − a²) + R² arcsin(a/R), R = 10, L = 40, a = 3, h = 15"
tex = '\pi R^2 L + 2a\left(2ah - S\right),\quad S = a\sqrt{R^2 - a^2} + R^2\arcsin\tfrac{a}{R},\quad R = 10,\ L = 40,\ a = 3,\ h = 15'

[area]
value = 3268.381948049816
exact = "2πR² + 2πRL − 4aR arcsin(a/R) + 4a² + 2(2ah − S) + 4a(h − √(R² − a²)), with S as for the volume"
tex = '2\pi R^2 + 2\pi RL - 4aR\arcsin\tfrac{a}{R} + 4a^2 + 2\left(2ah - S\right) + 4a\left(h - \sqrt{R^2 - a^2}\right)'
+++

A cylinder of radius 10 lying along the x axis, from x = −20 to x = 20, with a square pad on top.
The pad is the square |x| ≤ 3, |y| ≤ 3 in the plane z = 15, extruded down (along −z) until it
meets the cylinder's side, and joined to the cylinder.

So the pad's top is that square, its four sides are vertical, and its bottom is the part of the
cylinder's side under the square: it reaches z = 10 at its middle, y = 0, and z = √91 ≈ 9.539 at
its sides y = ±3. The pad and the cylinder are one solid.
