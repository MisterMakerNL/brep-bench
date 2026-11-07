+++
title = "An elliptic cylinder shrunk past its curvature"
tags = ["offset", "shrink", "ellipse", "self-intersection"]
settled = false
+++

The elliptic cylinder of `geometry/elliptic-cylinder`, the ellipse x²/25 + y²/9 ≤ 1 in the plane
z = 0 extruded straight up to z = 10, shrunk by 2.5: the solid of every point at least 2.5 from
its outside.

The ends move to z = 2.5 and z = 7.5. The side would move in along its normals by 2.5, but the
ellipse curves more tightly than that near the ends of its long axis, where its radius of
curvature falls to b²/a = 1.8: there the curve moved in by 2.5 turns back on itself in a
swallowtail, two cusps and a loop crossing the long axis. Of that curve, only the part at least
2.5 from the ellipse everywhere bounds the result. The loops are trimmed away where the curve
crosses itself on the long axis, and the shrunk side meets itself there in a sharp vertical edge
at each end, from z = 2.5 to z = 7.5.

The trimmed curve's area and length have no closed form here, so the case is unsettled until two
kernels agree.
