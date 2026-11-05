+++
title = "A drum under a saddle, cut level through the saddle point"
tags = ["singular", "nurbs", "bspline", "saddle", "boolean", "cut", "tangent-crossing", "ssi"]
origin = "jyviko/ssi-corpus c4n_saddle_tangent_plane"

[volume]
value = 1882.6192982974677
exact = "the drum 640π less what stands above z = 10, ∫∫ xy/8 where xy > 0 = 128: 640π − 128"
tex = '640\pi - \tfrac18\iint_{xy > 0} xy\,dA = 640\pi - 128'

[area]
value = 862.79008140382844
exact = "the floor 64π + the side 160π − 64 + the flat top over xy > 0, 32π, + the saddle over xy < 0, half of (128π/3)(2√2 − 1)"
tex = '64\pi + (160\pi - 64) + 32\pi + \tfrac{64\pi}{3}\left(2\sqrt 2 - 1\right)'
+++

The drum of `geometry/saddle-drum`, its floor flat at z = 0, its side the cylinder of radius 8
about the z axis and its top the saddle z = 10 + xy/8, made as that case says, as the bilinear
B-spline patch on the corners of the square −10 ≤ x, y ≤ 10. Cut away everything above the plane
z = 10, and keep what lies on and below it: every point with x² + y² ≤ 64 and
0 ≤ z ≤ min(10, 10 + xy/8). If you cut with a block, make it reach past the drum on every side
but its floor.

The plane z = 10 is tangent to the saddle at its saddle point, (0, 0, 10), the middle of the
patch, and meets it in two straight lines through that point, the x and y axes raised to z = 10,
which cross there. Over the two quarters where xy > 0 the saddle rises above the plane and is cut
off, leaving two flat quarter disks; over the two where xy < 0 it falls below the plane and stays.
The top is four faces meeting at the saddle point, flat and saddle alternating, and the side falls
to 6 above the diagonal x = −y and is cut level at 10 elsewhere.
