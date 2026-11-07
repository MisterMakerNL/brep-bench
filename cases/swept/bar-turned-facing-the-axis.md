+++
title = "A curved block cut by a bar's side facing the axis squarely"
tags = ["swept", "solid-sweep", "turn", "prism", "ruling", "cone", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/facing.sv"

[volume]
value = 37.745354108234779
exact = "(π/6)∫((10 + z tan 20°)² − 8²) dz over |z| ≤ 1 = (π/6)(72 + (2/3)tan²20°)"
tex = '\tfrac{\pi}{6}\int_{-1}^{1}\left((10 + z\tan 20^\circ)^2 - 8^2\right)dz = \tfrac{\pi}{6}\left(72 + \tfrac{2}{3}\tan^2 20^\circ\right)'

[area]
value = 84.881086605187371
exact = "top (π/6)((10 + t)² − 64) + bottom (π/6)((10 − t)² − 64) + inner wall 8·(π/3)·2 + the cone's sixth π·20·(2/cos 20°)/6 + two trapezoids 2·4, t = tan 20°"
tex = '\tfrac{\pi}{6}\left((10 + t)^2 - 64\right) + \tfrac{\pi}{6}\left((10 - t)^2 - 64\right) + \tfrac{16\pi}{3} + \tfrac{20\pi}{6\cos 20^\circ} + 8,\quad t = \tan 20^\circ'
+++

A curved block, cut by the inner side of a bar turned past it about the block's own axis.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is a box leaning out from upright in the plane y = 0. Its inner side is the plane through
(10, 0, 0) square to the direction (cos 20°, 0, −sin 20°), so it faces the z axis squarely and leans
out 20° from upright: at height z it stands 10 + z·tan 20° from the axis. The bar is everything from
that plane to 3 beyond it along (cos 20°, 0, −sin 20°), within 2 of the plane y = 0, and within 3 of
the plane through (10, 0, 0) square to (sin 20°, 0, cos 20°) (its length runs along that direction).
So it stands past the block above and below, and its outer side lies beyond the block's outer wall.

The bar turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it
starts clear of the block, passes through it, and ends clear of it. Cut from the block everything
the bar passes through.

Every line of the inner side that runs along the bar's length is in contact all along it or not at
all; the one in the plane through the axis cuts the block's new wall, a cone. What is left of the
block is the part of it within 10 + z·tan 20° of the axis.
