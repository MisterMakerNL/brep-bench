"""Two half balls of one ball, overlapping. The halves z ≥ 0 and x ≥ 0 of the ball of radius 5
about the origin, united: the first's poles on the z axis and the second's on the x axis."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5


def half(axis):
    """The half of the ball on the + side of the plane square to `axis` through the origin, its
    poles on that axis."""
    # OCCT's sphere has its poles on the z axis of its placement: here `axis`
    ball = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(*axis)), RADIUS).Shape()
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp_Dir(*axis))).Face()
    behind = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(*(-c for c in axis))).Solid()
    cut = BRepAlgoAPI_Cut(ball, behind)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined to halve the ball")
    return cut.Shape()


def build():
    fuse = BRepAlgoAPI_Fuse(half((0, 0, 1)), half((1, 0, 0)))
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two halves")
    return fuse.Shape()
