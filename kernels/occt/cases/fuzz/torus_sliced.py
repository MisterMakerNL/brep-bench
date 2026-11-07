"""A torus sliced across its axis, twenty ways. A torus about the z axis, radii R and r, the part
of it above z = h, turned by ψ about the axis (θ, φ)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, turned


def build(p):
    R, r, h = p["R"], p["r"], p["h"]
    torus = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), R, r).Shape()
    # Everything below z = h, and well past the torus on every other side.
    s = 2 * (R + r)
    below = BRepPrimAPI_MakeBox(gp_Pnt(-s, -s, -2 * r), 2 * s, 2 * s, h + 2 * r).Shape()
    cut = BRepAlgoAPI_Cut(torus, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slice")
    return turned(cut.Shape(), p["theta"], p["phi"], p["psi"])
