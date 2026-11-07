"""A slab drilled through aslant, twenty ways. A slab W by D by T, drilled r along the line through
(0, 0, T/2) tilted β from the vertical toward the azimuth φ."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, direction


def build(p):
    W, D, T = p["W"], p["D"], p["T"]
    slab = BRepPrimAPI_MakeBox(gp_Pnt(-W / 2, -D / 2, 0), W, D, T).Shape()
    d = direction(p["beta"], p["phi"])
    # The drill runs from well below the slab to well above it, along its axis.
    reach = W + D + T
    start = gp_Pnt(*(c - reach * x for c, x in zip((0, 0, T / 2), d)))
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(start, gp_Dir(*d)), p["r"], 2 * reach).Shape()
    cut = BRepAlgoAPI_Cut(slab, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
