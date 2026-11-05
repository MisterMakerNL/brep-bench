"""A paraboloid's tip a ten-thousandth high. The solid paraboloid x² + y² ≤ 8z, z ≤ 8, in common
with the half-space below the plane z = 10⁻⁴: its tip, whose rim is a circle of radius about
0.0283 around the vertex."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfParabola
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Parab, gp_Pln, gp_Pnt

from bench import Refused

FOCAL = 2     # x² + y² = 4·FOCAL·z
H = 8         # the paraboloid's top, a flat disk at z = H
TIP = 1e-4    # the plane the tip is cut by


def paraboloid():
    """The solid paraboloid: the parabola x² = 8z of the xz plane, from its vertex to the rim,
    turned once about z."""
    # gp_Parab opens along its frame's x (here +z), and its frame's y is +x when its normal is −y
    parabola = gp_Parab(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(0, 0, 1)), FOCAL)
    vertex, rim, top = gp_Pnt(0, 0, 0), gp_Pnt((4 * FOCAL * H) ** 0.5, 0, H), gp_Pnt(0, 0, H)
    arc = GC_MakeArcOfParabola(parabola, vertex, rim, True).Value()
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(rim, top).Edge(),
                                   BRepBuilderAPI_MakeEdge(top, vertex).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()


def build():
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, TIP), gp.DZ_s())).Face()
    below = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, -1)).Solid()
    common = BRepAlgoAPI_Common(paraboloid(), below)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the paraboloid and the half-space")
    return common.Shape()
