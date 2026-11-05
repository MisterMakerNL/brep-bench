"""A ball in a paraboloid cup that fits its bottom. The solid paraboloid x² + y² ≤ 10z, z ≤ 5,
united with the ball of radius 5 about (0, 0, 5), whose curvature matches the cup's at its vertex:
the two touch there only, to more than first order."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol, BRepPrimAPI_MakeSphere
from OCP.GC import GC_MakeArcOfParabola
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Parab, gp_Pnt

from bench import Refused

FOCAL = 2.5   # x² + y² = 4·FOCAL·z
TOP = 5       # the cup's top, a flat disk at z = TOP
RADIUS = 5    # of the ball, about (0, 0, RADIUS)


def cup():
    """The solid paraboloid: the parabola x² = 10z of the xz plane, from its vertex to the rim,
    turned once about z."""
    # gp_Parab opens along its frame's x (here +z), and its frame's y is +x when its normal is −y
    parabola = gp_Parab(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(0, 0, 1)), FOCAL)
    vertex, rim, top = gp_Pnt(0, 0, 0), gp_Pnt((4 * FOCAL * TOP) ** 0.5, 0, TOP), gp_Pnt(0, 0, TOP)
    arc = GC_MakeArcOfParabola(parabola, vertex, rim, True).Value()
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(rim, top).Edge(),
                                   BRepBuilderAPI_MakeEdge(top, vertex).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()


def build():
    # OCCT's sphere has its poles on the z axis of its placement: its lower pole is the vertex
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, RADIUS), RADIUS).Shape()
    fuse = BRepAlgoAPI_Fuse(cup(), ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cup and the ball")
    return fuse.Shape()
