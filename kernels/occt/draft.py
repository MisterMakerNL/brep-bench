"""What the OCCT implementations of the draft cases share: drafting faces of a solid with
BRepOffsetAPI_DraftAngle, a box with its vertical edges rounded, and a box's sides picked.

DraftAngle turns each face it is given about its neutral curve, where the face crosses the neutral
plane, until it makes the given angle with the pull direction: a plane turns about a line, and a
cylinder whose axis is the pull direction becomes a cone through the same circle. A positive angle
leans the face toward the solid's inside as it runs along the pull direction (an outer side
narrows, a hole's wall widens), and a negative one away from it. The faces beside the drafted ones
are extended or trimmed to meet them.
"""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused, face_through


def drafted(solid, faces, degrees, neutral=(0, 0, 0), pull=(0, 0, 1)):
    """`solid` with each of `faces` drafted `degrees` about the plane through `neutral` square to
    `pull`. Refused if OCCT declines a face or the whole."""
    direction = gp_Dir(*pull)
    plane = gp_Pln(gp_Pnt(*neutral), direction)
    draft = BRepOffsetAPI_DraftAngle(solid)
    for k, face in enumerate(faces):
        draft.Add(face, direction, math.radians(degrees), plane)
        if not draft.AddDone():
            raise Refused(f"BRepOffsetAPI_DraftAngle declined face {k + 1} of {len(faces)} "
                          f"({status(draft)})")
    draft.Build()
    if not draft.IsDone():
        raise Refused("BRepOffsetAPI_DraftAngle could not build the drafted solid "
                      f"({status(draft)})")
    return draft.Shape()


def status(draft):
    """What DraftAngle could not recompute, from its Draft_ErrorStatus: a face, an edge or a
    vertex (`FaceRecomputation` and the like)."""
    return draft.Status().name.removeprefix("Draft_")


def rounded_box(lo, hi, radius):
    """The box from `lo` to `hi` with its four vertical edges, those whose ends differ only in z,
    rounded to `radius`."""
    box = BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()
    rounds = BRepFilletAPI_MakeFillet(box)
    edges = TopExp_Explorer(box, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        a, b = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
        if abs(a.X() - b.X()) < 1e-9 and abs(a.Y() - b.Y()) < 1e-9:
            rounds.Add(radius, edge)
        edges.Next()
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the vertical edges")
    return rounds.Shape()


def box_sides(solid, lo, hi, radius=0):
    """The faces of `solid` where the box from `lo` to `hi` has its four sides, picked halfway up
    each: its planes at their middles and, where its vertical edges are rounded to `radius`, the
    four rounds at theirs. A pocket's walls are picked by its box the same way."""
    (x0, y0, z0), (x1, y1, z1) = lo, hi
    xm, ym, z = (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2
    points = [(x0, ym, z), (x1, ym, z), (xm, y0, z), (xm, y1, z)]
    if radius:
        k = radius * (1 - 2 ** -0.5)  # a quarter round's middle, in from the box's corner
        points += [(x0 + k, y0 + k, z), (x1 - k, y0 + k, z),
                   (x1 - k, y1 - k, z), (x0 + k, y1 - k, z)]
    return [face_through(solid, p) for p in points]
