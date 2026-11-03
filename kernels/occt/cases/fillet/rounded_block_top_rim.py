"""A rounded block with its top rim rounded. A block from (−10, −6, 0) to (10, 6, 5) with its four
vertical edges rounded to radius 3; round the rim of its top face, eight edges each running on
tangent into the next, to radius 1."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused

HEIGHT = 5
CORNER = 3
ROUND = 1
TOLERANCE = 1e-7


def edges(shape):
    found = TopExp_Explorer(shape, TopAbs_EDGE)
    while found.More():
        yield TopoDS.Edge(found.Current())
        found.Next()


def ends(edge):
    curve = BRepAdaptor_Curve(edge)
    return curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -6, 0), gp_Pnt(10, 6, HEIGHT)).Shape()
    # the vertical edges: both ends over the same point
    corners = BRepFilletAPI_MakeFillet(block)
    for edge in edges(block):
        a, b = ends(edge)
        if abs(a.X() - b.X()) < TOLERANCE and abs(a.Y() - b.Y()) < TOLERANCE:
            corners.Add(CORNER, edge)
    corners.Build()
    if not corners.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the vertical edges")
    rounded = corners.Shape()
    # the top rim: every edge with both ends at z = 5; they make one tangent chain
    rim = BRepFilletAPI_MakeFillet(rounded)
    for edge in edges(rounded):
        a, b = ends(edge)
        if abs(a.Z() - HEIGHT) < TOLERANCE and abs(b.Z() - HEIGHT) < TOLERANCE:
            rim.Add(ROUND, edge)
    assert rim.NbContours() == 1, f"{rim.NbContours()} chains round the top"
    rim.Build()
    if not rim.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the top rim")
    return rim.Shape()
