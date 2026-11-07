"""Three bodies nested in each other's hollows. Balls of radius 10 at the origin, 8 at (1, 0, 0),
6 at (1, 1, 0), 4 at (1, 1, 1) and 2 at (1, 1.5, 1.5): the first less the second, joined with the
third less the fourth, joined with the fifth."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def ball(radius, centre):
    return BRepPrimAPI_MakeSphere(gp_Pnt(*centre), radius).Shape()


def done(op, what):
    if not op.IsDone():
        raise Refused(f"OCCT declined {what}")
    return op.Shape()


def build():
    outer = done(BRepAlgoAPI_Cut(ball(10, (0, 0, 0)), ball(8, (1, 0, 0))), "the outer hollow")
    middle = done(BRepAlgoAPI_Cut(ball(6, (1, 1, 0)), ball(4, (1, 1, 1))), "the middle hollow")
    both = done(BRepAlgoAPI_Fuse(outer, middle), "joining the hollow balls")
    return done(BRepAlgoAPI_Fuse(both, ball(2, (1, 1.5, 1.5))), "joining the innermost ball")
