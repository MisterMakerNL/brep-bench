+++
title = "A ball in a paraboloid cup that fits its bottom"
tags = ["tangent", "boolean", "union", "paraboloid", "sphere", "revolution", "pole", "point-contact", "ssi"]
origin = "jyviko/ssi-corpus c2n_quartic_contact_parabola"

[volume]
value = 654.49846949787359
exact = "the cup ∫ π·10z dz from 0 to 5 = 125π + the half ball above it 2π5³/3 = 625π/3"
tex = '\int_0^5 10\pi z\,dz + \tfrac23\pi\cdot 5^3 = \tfrac{625\pi}{3}'

[area]
value = 455.32947609453728
exact = "the cup's side 2π∫ρ√(1 + ρ²/25) dρ from 0 to √50 = (50π/3)(3√3 − 1) + its top less the ball's 50π − 25π + the half sphere 50π"
tex = '\tfrac{50\pi}{3}\left(3\sqrt 3 - 1\right) + 50\pi - 25\pi + 50\pi'
+++

A solid paraboloid of revolution about the z axis, a cup with its vertex at the origin, filled up
to the plane z = 5: every point with x² + y² ≤ 10z and z ≤ 5. Unite it with a ball of radius 5
centered at (0, 0, 5).

The paraboloid's curvature at its vertex is 1/5, the sphere's: the ball's bottom fits the bottom
of the cup. The sphere touches the paraboloid only at the origin, but there the two agree to more
than first order: the sphere rises above the cup's side by only about ρ⁴/1000 at a distance ρ from
the axis (10⁻⁷ at ρ = 0.1). The lower half of the ball lies inside the cup, and its upper half
stands out of the cup's top through the circle of radius 5, the ball's equator. The union is the
cup with a half ball on its top. The point of contact is not a vertex of the result.

This case is about that contact of high order, where a search for where the two surfaces meet sees
them nearly coincide over a whole patch. If the ball you make has poles, put them on the z axis,
so that its pole is the point of contact; if the paraboloid is the parabola turned about the z
axis, that is where the turned curve meets the axis.
