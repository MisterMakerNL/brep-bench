"""A ball halved through its poles. A ball of radius 5 centred on the origin, with everything on
the side x < 0 of the yz plane cut away. Its poles on the z axis, at (0, 0, ±5), both in the
cutting plane."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5


def build():
    # OCCT's sphere has its poles on the z axis of its placement: here the global z axis
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    # everything with x < 0: the half-space on the side of a point with negative x
    yz_plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp.DX_s())).Face()
    behind = BRepPrimAPI_MakeHalfSpace(yz_plane, gp_Pnt(-1, 0, 0)).Solid()
    cut = BRepAlgoAPI_Cut(ball, behind)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
