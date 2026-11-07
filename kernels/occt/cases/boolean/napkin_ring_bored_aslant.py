"""A napkin ring bored aslant, off the origin. A ball of radius 6.528 centred on
(18.676, −15.427, 14.527), with a hole of radius 3.408 bored right through it along the line
through its centre in the direction (0.448, −0.297, −0.844)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

CENTRE = (18.676, -15.427, 14.527)
BALL_RADIUS = 6.528
HOLE_RADIUS = 3.408
DIRECTION = (0.448, -0.297, -0.844)


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(*CENTRE), BALL_RADIUS).Shape()
    # The drill runs well past the ball at both ends: from 2R before its centre to 2R after.
    length = math.hypot(*DIRECTION)
    along = [x / length for x in DIRECTION]
    start = gp_Pnt(*(c - 2 * BALL_RADIUS * x for c, x in zip(CENTRE, along)))
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(start, gp_Dir(*along)), HOLE_RADIUS,
                                     4 * BALL_RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(ball, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
