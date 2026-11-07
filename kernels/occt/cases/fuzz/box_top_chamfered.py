"""A box with its top edges chamfered unequally, twenty ways. A box from the origin to (a, b, c),
the four edges round its top chamfered d1 across the top and d2 down the sides, turned by ψ about
the axis (θ, φ)."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused, turned

TOLERANCE = 1e-7


def top_face(box, height):
    """The box's face z = height."""
    faces = TopExp_Explorer(box, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        plane = BRepAdaptor_Surface(face).Plane()
        if (abs(abs(plane.Axis().Direction().Z()) - 1) < TOLERANCE
                and abs(plane.Location().Z() - height) < TOLERANCE):
            return face
        faces.Next()
    raise AssertionError("no face at the top")


def build(p):
    box = BRepPrimAPI_MakeBox(p["a"], p["b"], p["c"]).Shape()
    top = top_face(box, p["c"])
    chamfer = BRepFilletAPI_MakeChamfer(box)
    edges = TopExp_Explorer(top, TopAbs_EDGE)
    while edges.More():
        chamfer.Add(p["d1"], p["d2"], TopoDS.Edge(edges.Current()), top)
        edges.Next()
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the edges round the top")
    return turned(chamfer.Shape(), p["theta"], p["phi"], p["psi"])
