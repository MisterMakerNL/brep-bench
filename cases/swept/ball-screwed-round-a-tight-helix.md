+++
title = "A ball screwed round a helix tighter than itself"
tags = ["swept", "solid-sweep", "screw", "canal", "helix", "fold", "self-intersecting"]
origin = "geomsolver #721, box X8 (folds cut where they lie)"
settled = false
+++

The solid a ball fills as its center runs along a helix whose curvature is tighter than the ball, so
that the tube about the helix folds through itself.

The ball has radius 2. Its center starts at (1.5, 0, 0) and runs along the helix of radius 1.5 about
the z axis that turns right-handed (from +x toward +y) and rises 3 every turn, for two whole turns,
ending at (1.5, 0, 6). Make the solid of everything the ball passes through.

The helix's radius of curvature, (1.5² + (3/2π)²)/1.5 ≈ 1.65, is less than the ball's radius, so the
envelope of the balls folds: its inner side is cut away by the balls themselves. And the turns, 3
apart, overlap. The solid is everything within 2 of the helix's arc.
