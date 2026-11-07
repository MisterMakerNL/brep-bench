"""A hoof cut from a cylinder through a diameter of its base. A cylinder of radius 5 on the z axis
from z = −1 to 10, the part above z = 0 and below z = x tan 30°."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeHalfSpace
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5
SLOPE = math.radians(30)


def half_space(normal, inside):
    """The half-space bounded by the plane through the origin square to `normal`, on the side of
    the point `inside`."""
    face = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp_Dir(*normal))).Face()
    return BRepPrimAPI_MakeHalfSpace(face, gp_Pnt(*inside)).Solid()


def build():
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), RADIUS, 11).Shape()
    above_base = half_space((0, 0, 1), (0, 0, 1))
    # the plane z = x tan 30°, and below it the point (1, 0, 0)
    below_top = half_space((-math.sin(SLOPE), 0, math.cos(SLOPE)), (1, 0, 0))
    step = BRepAlgoAPI_Common(cylinder, above_base)
    if not step.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the base")
    hoof = BRepAlgoAPI_Common(step.Shape(), below_top)
    if not hoof.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the slanting top")
    return hoof.Shape()
