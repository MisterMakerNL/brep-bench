"""A tee of two cylinders with its junction rounded. The cylinder of radius 6 about the x axis
from x = -20 to x = 20 joined with the cylinder of radius 3 about the z axis from z = 0 to z = 15,
the saddle-shaped curve where their sides meet rounded to radius 1."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def on_junction(edge):
    """Whether `edge`'s middle lies on both cylinders' sides."""
    curve = BRepAdaptor_Curve(edge)
    p = curve.Value((curve.FirstParameter() + curve.LastParameter()) / 2)
    return (abs(p.X() ** 2 + p.Y() ** 2 - 9) < 1e-6 and abs(p.Y() ** 2 + p.Z() ** 2 - 36) < 1e-6)


def build():
    bar = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-20, 0, 0), gp_Dir(1, 0, 0)), 6, 40).Shape()
    stem = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 3, 15).Shape()
    fuse = BRepAlgoAPI_Fuse(bar, stem)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the stem to the bar")
    tee = fuse.Shape()
    fillet = BRepFilletAPI_MakeFillet(tee)
    found = []
    edges = TopExp_Explorer(tee, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if on_junction(edge) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
            fillet.Add(1, edge)
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the junction")
    return fillet.Shape()
