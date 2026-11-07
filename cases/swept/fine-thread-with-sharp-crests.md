+++
title = "A thread so fine its crests are sharp"
tags = ["swept", "solid-sweep", "screw", "helix", "prism", "helicoid", "thin", "boolean", "cut"]
origin = "geomsolver #721, box F4 (precision at small leads)"

[volume]
value = 1276.7029992078067
exact = "π ρf²·20 − 20·2π∫ ρ w(ρ) dρ from 4.5 to ρf, w(ρ) = 0.9 + acos(4.5/ρ)/π, ρf = 4.5/cos(π/10) where w = 1 (beyond it the rod is wholly cut away)"

[area]
value = 916.12039648761102
exact = "the root 2π·4.5·20·0.9 + two ends 2(π ρf² − 2π∫ ρ w dρ) + two helicoidal walls 2(20/c)·½[√(U(U + c²)) + c² ln((√U + √(U + c²))/c)], c = 1/2π, U = ρf² − 4.5²"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar carried
along a screw so fine that the turns of the groove meet: the rod's own surface is cut away
everywhere, and what is left between neighbouring turns is a thread whose crest is a sharp helical
edge.

The bar, as placed, is the box 4.5 ≤ x ≤ 12, −8 ≤ y ≤ 8, −0.45 ≤ z ≤ 0.45: it points at the rod's
axis, its flat tip (the face x = 4.5) 0.5 inside the rod, and it is 0.9 wide along the axis.

The screw is about the z axis: the bar turns right-handed about it (from +x toward +y) and rises 1
for every whole turn, rigidly. It runs from a turn of −180° (the bar lowered 0.5, wholly below the
rod) to a turn of 7380° (raised 20.5, wholly above it). Cut from the rod everything the bar passes
through.

At distance ρ from the axis the groove is 0.9 + acos(4.5/ρ)/π wide along it (its walls are where the
tip's long edges pass). That reaches the whole lead, 1, at ρ = 4.5/cos 18° ≈ 4.7316: there the walls
of neighbouring turns meet in the crest, and beyond it nothing of the rod is left. Each turn of the
thread is about 0.23 deep, 0.1 wide at its root and sharp at its crest.
