"""A round pipe mitred along three edges of a cube. A pipe of circular section swept along three
edges of a cube, mitred at its two corners.

The path: three straight runs along three edges of a cube of side 30, from (0, 0, 0) to
(30, 0, 0), to (30, 30, 0), to (30, 30, 30). Each corner is a right angle, and the second turns
out of the plane of the first two runs.

The section: a disc of radius 2, square to the path and centred on it.

Each run is the disc swept straight along it, and at each corner the two runs are mitred: each
is cut by, and they meet in, the plane through the corner that bisects it, square to the sum of
the two runs' directions. At (30, 0, 0) that is the plane x + y = 30, and at (30, 30, 0) the
plane y + z = 30; each mitre is an ellipse in its plane, with half-axes 2 and 2√2. The pipe's
ends are flat and square to the path, at (0, 0, 0) and at (30, 30, 30)."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from corner import MITRE, polyline, swept

SIDE, RADIUS = 30, 2


def build():
    path = polyline([(0, 0, 0), (SIDE, 0, 0), (SIDE, SIDE, 0), (SIDE, SIDE, SIDE)])
    # The disc's rim at the path's start, square to the first run.
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), RADIUS))
    return swept(path, BRepBuilderAPI_MakeWire(circle.Edge()).Wire(), MITRE)
