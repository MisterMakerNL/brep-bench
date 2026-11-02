"""A picture frame: a rebated moulding mitred round a rectangle. A picture frame: a moulding swept
round a rectangle, mitred at its four corners.

The path: the rectangle in the xy plane with corners (±30, ±20, 0), 60 × 40, closed.

The section stands square to the path. It is given by s, the distance outward from the path
(away from the rectangle's centre), and z: the polygon through (s, z) = (0, 2), (3, 2), (3, 0),
(8, 0), (8, 6), (0, 6). That is a bar 8 wide and 6 tall lying outside the path, with a rebate 3
wide and 2 deep taken out of its inner bottom edge, where a picture's glass would sit. On the
side y = −20, for example, the point (s, z) of the section is at (x, −20 − s, z).

Each side of the rectangle is the section swept straight along it, and at each corner the two
sides are mitred: each is cut by, and they meet in, the vertical plane through the corner along
its diagonal, at 45° to both sides. At (30, −20, 0) that is the plane x + y = 10, running from the
path's corner out to the frame's outer corner at (38, −28). The path is closed, so the frame has
no ends: it is a ring 76 × 56 outside, with an opening 60 × 40 above the rebate and 66 × 46
below it."""
from OCP.gp import gp_Dir

from corner import MITRE, polyline, swept

HALF_WIDTH, HALF_HEIGHT = 30, 20  # the path's rectangle
SECTION = [(0, 2), (3, 2), (3, 0), (8, 0), (8, 6), (0, 6)]  # (s outward, z)


def build():
    # Round the rectangle anticlockwise from the middle of its side y = −20, where the section
    # stands: there, outward is −y, so the point (s, z) is at (0, −20 − s, z).
    w, h = HALF_WIDTH, HALF_HEIGHT
    path = polyline([(0, -h, 0), (w, -h, 0), (w, h, 0), (-w, h, 0), (-w, -h, 0)], closed=True)
    section = polyline([(0, -h - s, z) for s, z in SECTION], closed=True)
    return swept(path, section, MITRE, binormal=gp_Dir(0, 0, 1))  # the section kept upright
