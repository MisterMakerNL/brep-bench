"""A lens of two unequal balls, one placed aslant. Balls of radius 5.511 at the origin and 3.317 at
(4.231, 3.467, 2.653); the lens common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5.511).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(4.231, 3.467, 2.653), 3.317).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the lens")
    return common.Shape()
