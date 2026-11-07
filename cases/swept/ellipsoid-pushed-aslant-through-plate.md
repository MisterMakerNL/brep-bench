+++
title = "An ellipsoid pushed aslant wholly through a plate"
tags = ["swept", "solid-sweep", "slide", "ellipsoid", "quadric", "oblique", "elliptic-integral", "boolean", "cut"]
origin = "geomsolver #721, box D4 (a closed characteristic with no crease)"

[volume]
value = 3522.2654176407504
exact = "3600 − 4·π√(b²c²u₁² + a²c²u₂² + a²b²u₃²)·|d|/|d_z|, (a, b, c) = (3, 2, 1.5), u = d/|d|, d = (4, 3, 12): each level loses the ellipsoid's shadow along the push"

[area]
value = 2307.9842959944004
exact = "2280 − 2·(that shadow) + 4|d|/|d_z|·4p E(1 − q²/p²), p, q the semi-axes of the ellipsoid's shadow on the plane square to d (the square roots of the eigenvalues of P diag(a², b², c²) P, P the projection across d) and E the complete elliptic integral of the second kind (parameter m)"
+++

A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched through it by an ellipsoid
pushed aslant wholly through it.

The ellipsoid has semi-axes 3, 2 and 1.5 along x, y and z: everything with ((x + 2)/3)² + ((y +
1.5)/2)² + ((z + 4)/1.5)² ≤ 1, centered at (−2, −1.5, −4), wholly below the plate.

It slides without turning along the vector (4, 3, 12), ending centered at (2, 1.5, 8), wholly above
the plate. Cut from the plate everything the ellipsoid passes through.

The ellipsoid touches what it sweeps along one smooth closed curve, where its normal is square to
the push. That curve's path is the hole's wall, an oblique elliptic cylinder: every level of the
plate loses the same ellipse.
