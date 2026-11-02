"""A square bar mitred round a sharp corner. A square bar swept along a path with one sharp corner,
mitred there.

The path: two straight runs, from (0, 0, 0) along +x to (40, 0, 0), then turning 120° to the
left (towards +y) and on 40 to (20, 20√3, 0). So the runs meet at (40, 0, 0) at 60°.

The section: a square 10 × 10, square to the path and centred on it, two of its sides
horizontal, at z = −5 and z = 5.

Each run is the square swept straight along it, and at the corner the two are mitred: each is
cut by, and they meet in, the vertical plane through the corner that bisects it,
x + √3y = 40. The mitre is a rectangle 20 long and 10 tall in that plane, from the inside of the
corner, at x = 40 − 5√3 ≈ 31.340 on the first run's inner side (y = 5), to its outside, at
x = 40 + 5√3 ≈ 48.660 on the first run's outer side (y = −5). The bar's ends are flat and square
to the path: in the plane x = 0, and in the plane through (20, 20√3, 0) square to the second run."""
import math

from OCP.gp import gp_Dir

from corner import MITRE, polyline, swept

RUN, HALF_SIDE = 40, 5


def build():
    turn = math.radians(120)  # to the left, from +x towards +y
    path = polyline([(0, 0, 0), (RUN, 0, 0),
                     (RUN + RUN * math.cos(turn), RUN * math.sin(turn), 0)])
    # The square at the path's start, in the plane x = 0, its sides at y, z = ±5.
    h = HALF_SIDE
    square = polyline([(0, -h, -h), (0, h, -h), (0, h, h), (0, -h, h)], closed=True)
    return swept(path, square, MITRE, binormal=gp_Dir(0, 0, 1))  # two sides kept horizontal
