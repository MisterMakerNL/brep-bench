+++
title = "A V groove cut by a sharp V wheel slid across a block"
tags = ["swept", "solid-sweep", "slide", "revolution", "sharp-edge", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/vgroove.sv"

[volume]
value = 1953.8119784648299
exact = "2000 − 20·2²·tan 30°"
tex = '2000 - 20\cdot 2^2\tan 30^\circ'

[area]
value = 1041.5692193816531
exact = "1000 − 20·4 tan 30° − 2·4 tan 30° + 2·20·2/cos 30°"
tex = '1000 - 88\tan 30^\circ + \tfrac{80}{\cos 30^\circ}'
+++

A block, the box from (0, −5, 0) to (20, 5, 10), with a V groove cut across its top by a wheel with
a sharp rim.

The wheel's axis is the line through (−8, 0, 14) parallel to the y axis. The wheel is the solid of
revolution, about that axis, of the triangle in the plane x = −8 with corners, in (y, z), at (0, 8),
(−3 tan 30°, 11) and (3 tan 30°, 11): its rim is a sharp circle of radius 6 whose lowest point, (−8,
0, 8), is 2 below the block's top, and its flanks lean 30° either side of its mid-plane (a 60° V),
back to its bore of radius 3.

It slides without turning along the x axis by 36, from clear before the block to clear past it. Cut
from the block everything the wheel passes through.

Each flank touches along its line square to the slide, and the two lines meet at the rim's lowest
point, where every normal between the flanks is the wheel's. The groove is the V of those lines
carried along the block: two planes meeting in a sharp edge 2 deep, the rim's lowest point's path.
