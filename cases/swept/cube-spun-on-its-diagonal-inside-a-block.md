+++
title = "A void spun by a cube turned about its long diagonal inside a block"
tags = ["swept", "solid-sweep", "turn", "whole-turn", "polyhedron", "hyperboloid", "cone", "void", "boolean", "cut"]
origin = "geomsolver #721, box D4 (a prism turned wholly within)"

[volume]
value = 201.48960508612626
exact = "6³ − 8π/√3: the cube's shadow about its diagonal is two cones and between them a hyperboloid of one sheet, radius² 2 + 2h² at h along the diagonal"
tex = '6^3 - \tfrac{8\pi}{\sqrt 3}'

[area]
value = 248.29761691689097
exact = "6·6² + two cones 2·π√(8/3)·2 + the hyperboloid 2π∫√(2 + 6h²) dh over |h| ≤ 1/√3 = 2π√(2/3)(√2 + asinh 1)"
tex = '216 + 4\pi\sqrt{\tfrac{8}{3}} + 2\pi\sqrt{\tfrac{2}{3}}\left(\sqrt 2 + \operatorname{asinh} 1\right)'
+++

A block, the box from (−3, −3, −3) to (3, 3, 3), with a closed void inside it: everything a cube
passes through as it turns a whole turn about its own long diagonal, wholly inside the block.

The cube is the box from (−1, −1, −1) to (1, 1, 1). It turns rigidly a whole turn about the line
through (−1, −1, −1) and (1, 1, 1), its long diagonal. Cut from the block everything the cube passes
through.

The cube's six edges that meet the diagonal's ends sweep two cones, and its six other edges, skew to
the diagonal, sweep a hyperboloid of one sheet between them, of waist radius √2. The void touches
nothing outside it: the result is the block with a sealed cavity, bounded by two shells.
