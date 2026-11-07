"""A cylinder with its top rim rounded, twenty ways. A cylinder of radius R on the z axis from 0 to
H, the edge round its top rounded to ρ, turned by ψ about the axis (θ, φ)."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, turned


def build(p):
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), p["R"], p["H"]).Shape()
    fillet = BRepFilletAPI_MakeFillet(cylinder)
    # The circle at z = H.
    edges = TopExp_Explorer(cylinder, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and abs(curve.Circle().Location().Z() - p["H"]) < 1e-9):
            fillet.Add(p["rho"], edge)
            break
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the rim")
    return turned(fillet.Shape(), p["theta"], p["phi"], p["psi"])
