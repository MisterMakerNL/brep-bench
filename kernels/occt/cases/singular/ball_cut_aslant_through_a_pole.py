"""A ball cut aslant through its pole. A ball of radius 5 centred on the origin, its poles on the
z axis, cut by the plane x + z = 5 through its pole at (0, 0, 5); the cap where x + z > 5 cut
away."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    # the plane through the pole, square to (1, 0, 1); the cap is on the side of (5, 0, 5)
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, RADIUS), gp_Dir(1, 0, 1))).Face()
    beyond = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(RADIUS, 0, RADIUS)).Solid()
    cut = BRepAlgoAPI_Cut(ball, beyond)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
