+++
title = "A ball's cap a ten-thousandth high, around its pole"
tags = ["tangent", "boolean", "common", "analytic", "sphere", "pole", "near-degenerate", "sliver", "ssi"]
origin = "jyviko/ssi-corpus c7_tiny_loop_sphere_1e-4"

[volume]
value = 1.5707858548193847e-07
exact = "πh²(3R − h)/3, R = 5, h = 10⁻⁴"
tex = '\tfrac{\pi h^2}{3}(3R - h),\quad R = 5,\ h = 10^{-4}'

[area]
value = 0.0062831538912530506
exact = "the cap 2πRh + the disk π(2Rh − h²), R = 5, h = 10⁻⁴"
tex = '2\pi R h + \pi\left(2Rh - h^2\right),\quad R = 5,\ h = 10^{-4}'
+++

A ball of radius 5 centered on the origin, and the half-space above the plane z = 4.9999, which
lies 10⁻⁴ below the ball's top. Make the solid common to both: the cap of the ball 10⁻⁴ high,
every point of the ball with z ≥ 4.9999. If you cut with a block, make it reach past the ball on
every side but its floor.

The plane meets the sphere in a circle of radius √(2·5·10⁻⁴ − 10⁻⁸) ≈ 0.0316, the cap's rim,
where the sphere leaves the plane at only about 0.36°. The cap is a disk 0.063 across under a
dome 10⁻⁴ high.

This case is about finding that small loop: a search for where the plane meets the sphere that
samples the sphere coarsely can step right over it and find nothing, or find only the point where
the plane would touch the sphere. If the ball you make has poles, put them on the z axis, so that
the loop runs around the pole at (0, 0, 5).
