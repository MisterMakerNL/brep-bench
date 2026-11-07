"""A lens of two unequal balls, twenty ways. Balls of radius r1 at the origin and r2 at d·(θ, φ);
the lens common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused, direction


def build(p):
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), p["r1"]).Shape()
    centre = gp_Pnt(*(p["d"] * x for x in direction(p["theta"], p["phi"])))
    second = BRepPrimAPI_MakeSphere(centre, p["r2"]).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the lens")
    return common.Shape()
