"""A round pipe turned round a sharp corner with a round joint. A pipe of circular section swept
along a path with one sharp corner, the section carried round the corner by turning it about the
corner.

The path: two straight runs, from (−30, 0, 0) along +x to the origin, then turning a right angle
to the left and on along +y to (0, 30, 0).

The section: a disc of radius 3, square to the path and centred on it.

Each run is the disc swept straight along it, ending flat at the corner, in the plane through
the origin square to the run. Inside the corner the two runs overlap. The wedge they leave open
outside the corner (x ≥ 0, y ≤ 0) is filled by the disc turned about the vertical line through
the corner, the z axis, a quarter turn from square to the first run to square to the second:
that is a quarter of the ball of radius 3 about the origin. So the pipe is every point within 3
of the path, cut off flat at its ends, (−30, 0, 0) and (0, 30, 0), by the planes there square to
the path."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from corner import ROUND, polyline, swept

RUN, RADIUS = 30, 3


def build():
    path = polyline([(-RUN, 0, 0), (0, 0, 0), (0, RUN, 0)])
    # The disc's rim at the path's start, square to the first run.
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(-RUN, 0, 0), gp_Dir(1, 0, 0)), RADIUS))
    return swept(path, BRepBuilderAPI_MakeWire(circle.Edge()).Wire(), ROUND)
