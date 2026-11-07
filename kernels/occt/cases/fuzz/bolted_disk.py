"""A disk bored and drilled on a bolt circle, twenty ways. A disk of radius Rd and thickness T,
bored rb along its axis and drilled n times rh on the circle of radius Rb from angle φ0, turned by
ψ about the axis (θ, φ)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, turned


def drill(x, y, radius, thickness):
    """A drill along z through (x, y), running past both faces."""
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, -1), gp.DZ_s()), radius,
                                    thickness + 2).Shape()


def build(p):
    disk = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), p["Rd"], p["T"]).Shape()
    tools = [drill(0, 0, p["rb"], p["T"])]
    for i in range(p["n"]):
        a = math.radians(p["phi0"]) + 2 * math.pi * i / p["n"]
        tools.append(drill(p["Rb"] * math.cos(a), p["Rb"] * math.sin(a), p["rh"], p["T"]))
    for tool in tools:
        cut = BRepAlgoAPI_Cut(disk, tool)
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a hole")
        disk = cut.Shape()
    return turned(disk, p["theta"], p["phi"], p["psi"])
