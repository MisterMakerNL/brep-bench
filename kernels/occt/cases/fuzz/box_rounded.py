"""A box with every edge rounded, twenty ways. A box from the origin to (a, b, c), its twelve edges
rounded to radius ρ, turned by ψ about the axis (θ, φ)."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused, turned


def build(p):
    box = BRepPrimAPI_MakeBox(p["a"], p["b"], p["c"]).Shape()
    fillet = BRepFilletAPI_MakeFillet(box)
    edges = TopExp_Explorer(box, TopAbs_EDGE)
    while edges.More():
        fillet.Add(p["rho"], TopoDS.Edge(edges.Current()))
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the box's edges")
    return turned(fillet.Shape(), p["theta"], p["phi"], p["psi"])
