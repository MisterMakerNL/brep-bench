"""The Bézier dome block with one top edge rounded. The block of nurbs/bezier-dome, under its
bicubic Bézier patch, the edge where the patch meets the side y = 0 rounded to radius 1."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet

from bench import Refused, other_case
from edit import straight_edges


def build():
    block = other_case("nurbs/bezier-dome").build()
    # the patch's edge v = 0: both its ends on y = 0, above the floor
    edges = straight_edges(block, lambda p: abs(p.Y()) < 1e-9 and p.Z() > 1)
    fillet = BRepFilletAPI_MakeFillet(block)
    for edge in edges:
        fillet.Add(1, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the dome's edge")
    return fillet.Shape()
