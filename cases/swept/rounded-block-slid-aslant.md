+++
title = "A block with every edge rounded, slid aslant"
tags = ["swept", "solid-sweep", "slide", "rounded", "fillet", "convex", "zonotope", "steiner"]
origin = "geomsolver #721, box G2 (rounded tools)"

[volume]
value = 694.65445273044353
exact = "Steiner's formula over the zonotope Z (the 8 × 4 × 2 box slid by d): V(Z) + S(Z)r + π(8 + 4 + 2 + |d|)r² + 4πr³/3, r = 1, d = (8, 5, 3), V(Z) = abc + |dx|bc + |dy|ca + |dz|ab"
tex = 'V(Z) + S(Z)\,r + \pi(a + b + c + |d|)\,r^2 + \tfrac{4}{3}\pi r^3,\quad (a, b, c) = (8, 4, 2),\ r = 1,\ d = (8, 5, 3)'

[area]
value = 474.11451085738198
exact = "S(Z) + 2π(8 + 4 + 2 + |d|)r + 4πr², S(Z) = 2(ab + bc + ca) + 2(a|e₁×d| + b|e₂×d| + c|e₃×d|), (a, b, c) = (8, 4, 2)"
tex = 'S(Z) + 2\pi(a + b + c + |d|)\,r + 4\pi r^2'
+++

The solid a rounded block fills as it slides along a straight line without turning.

The block is the box from (0, 0, 0) to (10, 6, 4) with every edge rounded to radius 1 and every
corner to a ball of radius 1: everything within 1 of the box from (1, 1, 1) to (9, 5, 3). Its faces
are six flat rectangles, twelve quarter cylinders and eight eighths of a ball.

It slides along the vector (8, 5, 3), ending as the same rounded block moved by (8, 5, 3). Make the
solid of everything the block passes through.

The rounded faces touch along their lines and arcs where their normals are square to the slide, and
those paths are what the flat sides of a box slid aslant would sweep, rounded: the result is
everything within 1 of the box from (1, 1, 1) to (9, 5, 3) slid along (8, 5, 3).
