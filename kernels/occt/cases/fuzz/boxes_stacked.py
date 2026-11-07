"""A box stood on part of another, joined, twenty ways. A box from the origin to (a1, b1, c1) and a
box a2 by b2 by c2 standing on its top face at (x2, y2), joined, turned by ψ about the axis
(θ, φ)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused, turned


def build(p):
    lower = BRepPrimAPI_MakeBox(p["a1"], p["b1"], p["c1"]).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(p["x2"], p["y2"], p["c1"]), p["a2"], p["b2"],
                                p["c2"]).Shape()
    fuse = BRepAlgoAPI_Fuse(lower, upper)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boxes")
    fuse.SimplifyResult()  # the faces the boxes share in one plane, made one
    return turned(fuse.Shape(), p["theta"], p["phi"], p["psi"])
