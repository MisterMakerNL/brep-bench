+++
title = "A paraboloid's tip a ten-thousandth high"
tags = ["tangent", "boolean", "common", "paraboloid", "revolution", "pole", "near-degenerate", "sliver", "ssi"]
origin = "jyviko/ssi-corpus c7_tiny_loop_parab_1e-4"

[volume]
value = 1.2566370614359173e-07
exact = "∫ π·8z dz from 0 to h = 4πh², h = 10⁻⁴"
tex = '\int_0^h 8\pi z\,dz = 4\pi h^2,\quad h = 10^{-4}'

[area]
value = 0.0050265796614084106
exact = "the disk 8πh + the side 2π∫ρ√(1 + ρ²/16) dρ from 0 to √(8h) = (32π/3)((1 + h/2)^{3/2} − 1), h = 10⁻⁴"
tex = '8\pi h + \tfrac{32\pi}{3}\left(\left(1 + \tfrac{h}{2}\right)^{3/2} - 1\right),\quad h = 10^{-4}'
+++

The solid paraboloid of `geometry/paraboloid`, every point with x² + y² ≤ 8z and z ≤ 8, and the
half-space below the plane z = 10⁻⁴. Make the solid common to both: the paraboloid's tip,
every point with x² + y² ≤ 8z and z ≤ 10⁻⁴. If you cut with a block, make it reach past the
paraboloid on every side but its top.

The plane meets the paraboloid in a circle of radius √(8·10⁻⁴) ≈ 0.0283 about the z axis, the
tip's rim, where the paraboloid leaves the plane at about 0.4°. The tip is a disk 0.057 across
over a bowl 10⁻⁴ deep.

This case is about finding that small loop on a surface no kernel meets in closed form unless it
knows paraboloids: a search that samples the surface coarsely can step over it and find nothing,
or only the vertex. If the paraboloid you make is the parabola turned about the z axis, the loop
runs around the vertex, where the turned curve meets the axis.
