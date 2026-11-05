"""A paraboloid's tip on a block's face. A block from (−10, −10, 0) to (10, 10, 10), united with
the solid paraboloid x² + y² ≤ 8z, z ≤ 12.5, whose vertex touches the middle of the block's bottom
face from inside."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfParabola
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Parab, gp_Pnt

from bench import Refused

HALF_SIDE = 10
HEIGHT = 10   # of the block
FOCAL = 2     # x² + y² = 4·FOCAL·z
TOP = 12.5    # the paraboloid's top, a flat disk of radius 10


def paraboloid():
    """The solid paraboloid: the parabola x² = 8z of the xz plane, from its vertex to the rim,
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
    block = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, HEIGHT)).Shape()
    fuse = BRepAlgoAPI_Fuse(block, paraboloid())
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block and the paraboloid")
    return fuse.Shape()
