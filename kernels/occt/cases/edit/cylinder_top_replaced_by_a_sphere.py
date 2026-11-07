"""A cylinder's flat top replaced by a dome. The cylinder of radius 6 about the z axis from z = 0 to
z = 10, its top's plane replaced by the sphere of radius 10 centred at (0, 0, 2), which meets the
side where the top's rim was: a cap of height 2 on the cylinder."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    # Workaround: OCCT has no operation that gives a face a new surface. The same solid is a
    # cylinder tall enough to hold the dome, cut down to the ball: their common part.
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 6, 20).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 2), 10).Shape()
    common = BRepAlgoAPI_Common(cylinder, ball)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not trim the cylinder to the ball")
    return common.Shape()
