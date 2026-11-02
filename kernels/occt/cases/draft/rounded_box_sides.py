"""A rounded box with its sides drafted, the rounds with them. The box from the origin to (30, 20,
10), with its four vertical edges rounded to radius 4: its sides are four planes and four quarter
cylinders of radius 4, about the vertical lines through (4, 4), (26, 4), (26, 16) and (4, 16), each
tangent to the planes beside it.

Draft all eight sides by 5°, with the pull direction +z and the neutral plane z = 0, the plane of
its floor, so that each leans 5° from the z direction, inward as it rises. Each plane turns about
its bottom edge. Each quarter cylinder becomes a quarter cone about the same axis, through the same
quarter circle at z = 0, its radius 4 − z·tan 5° at height z, so that it stays tangent to the
drafted planes beside it. The floor and the top are not drafted; the top is trimmed to the drafted
sides.

The result has ten faces. Its section at height z is the floor's outline, the rounded rectangle,
moved z·tan 5° inward: its straight sides inset z·tan 5° and its corners' radii 4 − z·tan 5° about
the same centres. At the top they are inset 10·tan 5° ≈ 0.875, the corners of radius about 3.125."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused
from draft import drafted
from shell import face_through

SIZE = (30, 20, 10)
RADIUS = 4
DRAFT = 5  # degrees, inward as the sides rise


def vertical_edges(box):
    """The box's four vertical edges: those whose ends differ only in z."""
    found = []
    edges = TopExp_Explorer(box, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        p, q = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
        if (abs(p.X() - q.X()) < 1e-9 and abs(p.Y() - q.Y()) < 1e-9
                and not any(edge.IsSame(e) for e in found)):
            found.append(edge)
        edges.Next()
    return found


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    rounds = BRepFilletAPI_MakeFillet(box)
    for edge in vertical_edges(box):
        rounds.Add(RADIUS, edge)
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the vertical edges")
    rounded = rounds.Shape()
    a, b, h = SIZE
    r, z = RADIUS, h / 2
    # the four planes, at their middles, and the four quarter cylinders, at their middles
    k = r * (1 - 2 ** -0.5)
    points = [(0, b / 2, z), (a, b / 2, z), (a / 2, 0, z), (a / 2, b, z),
              (k, k, z), (a - k, k, z), (a - k, b - k, z), (k, b - k, z)]
    return drafted(rounded, [face_through(rounded, p) for p in points], DRAFT)
