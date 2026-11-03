"""A cone halved through its apex. A solid cone on the z axis, its base the disc of radius 5 in the
xy plane and its apex at (0, 0, 10), with everything on the side x < 0 of the yz plane cut away."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeHalfSpace
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5
HEIGHT = 10


def build():
    # from the base (radius 5) up to the apex (radius 0)
    cone = BRepPrimAPI_MakeCone(RADIUS, 0, HEIGHT).Shape()
    yz_plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp.DX_s())).Face()
    behind = BRepPrimAPI_MakeHalfSpace(yz_plane, gp_Pnt(-1, 0, 0)).Solid()
    cut = BRepAlgoAPI_Cut(cone, behind)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
