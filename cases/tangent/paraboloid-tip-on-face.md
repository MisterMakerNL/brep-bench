+++
title = "A paraboloid's tip on a block's face"
tags = ["tangent", "boolean", "union", "paraboloid", "revolution", "pole", "point-contact", "ssi"]
origin = "jyviko/ssi-corpus c3n_parab_apex"

[volume]
value = 4706.8583470577035
exact = "the block 20·20·10 + the paraboloid above it ∫ π·8z dz from 10 to 12.5 = 4000 + 225π"
tex = '20\cdot 20\cdot 10 + \int_{10}^{12.5} 8\pi z\,dz = 4000 + 225\pi'

[area]
value = 1824.4951605529168
exact = "the block 1600 − the disk 80π + the top 100π + the side between ρ = √80 and ρ = 10, 2π∫ρ√(1 + ρ²/16) dρ = (4π/3)(29√29 − 48√6)"
tex = '1600 - 80\pi + 100\pi + \tfrac{4\pi}{3}\left(29\sqrt{29} - 48\sqrt 6\right)'
+++

A block from (−10, −10, 0) to (10, 10, 10), united with a solid paraboloid of revolution about the
z axis whose vertex is at the origin, the middle of the block's bottom face: every point with
x² + y² ≤ 8z and z ≤ 12.5. Its side is the paraboloid x² + y² = 8z, and its top a flat disk of
radius 10 at z = 12.5.

The paraboloid's vertex touches the block's bottom face from inside at a single point, where the
two are tangent: the paraboloid is smooth there, and rises from the face only as the square of the
distance from the vertex. Up to z = 10 the paraboloid lies inside the block (its radius there is
√80 ≈ 8.94); above it the paraboloid stands out of the top face, widening from radius √80 to 10 at
z = 12.5. The vertex is not a vertex of the result: the bottom face stays a whole square.

If the paraboloid you make is the parabola turned about the z axis, the point of contact is where
the turned curve meets the axis.
