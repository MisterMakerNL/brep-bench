"""A notched cube with its inside corner rounded. The cube from the origin to (20, 20, 20) less the
cube from (10, 10, 10) to (20, 20, 20), its three concave edges from the inside corner rounded to
radius 3 at once, the corner filled by an eighth of a sphere of radius 3 about (13, 13, 13)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet

from bench import Refused
from edit import box, straight_edges


def on_two_inside_faces(p):
    """Whether `p` lies on at least two of the notch's inside planes, x, y or z = 10."""
    return sum(abs(c - 10) < 1e-9 for c in (p.X(), p.Y(), p.Z())) >= 2


def build():
    cut = BRepAlgoAPI_Cut(box((0, 0, 0), (20, 20, 20)), box((10, 10, 10), (21, 21, 21)))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the notch")
    notched = cut.Shape()
    edges = straight_edges(notched, on_two_inside_faces)   # the three concave edges
    fillet = BRepFilletAPI_MakeFillet(notched)
    for edge in edges:
        fillet.Add(3, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the inside corner")
    return fillet.Shape()
