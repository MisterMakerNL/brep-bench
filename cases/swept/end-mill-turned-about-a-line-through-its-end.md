+++
title = "A pocket cut by an end mill turned about a line through its flat end"
tags = ["swept", "solid-sweep", "turn", "cylinder", "flat-contact", "boolean", "cut"]
origin = "geomsolver #721, box G6 (a flat swept across the turn's own axis)"

[volume]
value = 1906.7513385990563
exact = "2000 − 2·S, S the region the end's disc sweeps: πr² + 2θrd less what the turn covers twice near the axis, ((θ − 2π)(b² − (r − d)²)/2 + lens(b) − π(r − d)²), r = 3, d = 1.5, θ = 2π/3, b where the disc's half-angle about the axis is π − θ/2 (b² + 1.5b − 6.75 = 0), lens(b) the area of the disc within b of the axis"

[area]
value = 1249.3562859134043
exact = "1200 + 2·P, P the region's perimeter: (r + d)θ + 2r·acos((b² − d² − r²)/2dr)"
+++

A block, the box from (−10, −10, −5) to (10, 10, 0), with a pocket cut into its top by an end mill
turned about a line that passes through its own flat end, as spot facing is done off center.

The mill is a solid cylinder of radius 3 standing upright about the vertical line through (1.5, 0),
from z = −2 up to z = 4: its flat end is 2 below the block's top, and the z axis passes through that
end, 1.5 from its center.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 120°, so that its axis ends
on the vertical line through (1.5 cos 120°, 1.5 sin 120°). Cut from the block everything the mill
passes through.

The pocket's floor, z = −2, is the region the end's disc sweeps: the disc where it starts and where
it stops, and between them everything out to 4.5 from the axis. Near the axis the two discs and the
turn overlap, so the pocket's wall there is only the parts of the two discs' rims the other does not
cover.
