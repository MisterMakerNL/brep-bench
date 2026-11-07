"""A napkin ring with its ball's poles a hair outside its rims. A ball of radius 5 centred on the
origin, bored right through along the z axis by a hole of radius 3; the ball's poles on the line
along (3.1, 0, 4), each a little under a degree outside a rim."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 5
HOLE_RADIUS = 3


def build():
    # the sphere's poles lie on the main direction of its placement
    ball = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(3.1, 0, 4)), RADIUS).Shape()
    # a drill reaching past the ball both ways along z
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -2 * RADIUS), gp.DZ_s()),
                                     HOLE_RADIUS, 4 * RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(ball, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the drill")
    return cut.Shape()
