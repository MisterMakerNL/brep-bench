"""A cone cut aslant through its apex. A solid cone on the z axis, its base the disc of radius 5 in
the xy plane and its apex at (0, 0, 10), cut by the plane 4x + z = 10 through its apex; the side
where 4x + z > 10, away from the axis, cut away."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeHalfSpace
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5
HEIGHT = 10


def build():
    cone = BRepPrimAPI_MakeCone(RADIUS, 0, HEIGHT).Shape()
    # the plane through the apex, square to (4, 0, 1); the part cut away is on the side of (5, 0, 0)
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, HEIGHT), gp_Dir(4, 0, 1))).Face()
    beyond = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(RADIUS, 0, 0)).Solid()
    cut = BRepAlgoAPI_Cut(cone, beyond)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
