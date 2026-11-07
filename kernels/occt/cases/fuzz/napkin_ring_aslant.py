"""A napkin ring bored aslant, twenty ways. A ball of radius R centred on (cx, cy, cz), with a hole
of radius r bored right through it along the line through its centre with direction (θ, φ)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, direction


def build(p):
    centre = (p["cx"], p["cy"], p["cz"])
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(*centre), p["R"]).Shape()
    # The drill runs well past the ball at both ends: from 2R before its centre to 2R after.
    d = direction(p["theta"], p["phi"])
    start = gp_Pnt(*(c - 2 * p["R"] * x for c, x in zip(centre, d)))
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(start, gp_Dir(*d)), p["r"], 4 * p["R"]).Shape()
    cut = BRepAlgoAPI_Cut(ball, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
