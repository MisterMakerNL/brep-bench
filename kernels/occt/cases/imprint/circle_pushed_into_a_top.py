"""A circle imprinted on a top and pushed in. The box from the origin to (30, 20, 10), the circle of
radius 5 about (15, 10, 10) imprinted on its top, splitting it into a disk and the rest; the disk
is then pushed down by 3, its rim sweeping a wall, into the floor of a round pocket."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepFeat import BRepFeat_MakePrism, BRepFeat_SplitShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused, face_through


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(30, 20, 10)).Shape()
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(15, 10, 10), gp_Dir(0, 0, 1)), 5)).Edge()
    imprint = BRepFeat_SplitShape(block)
    imprint.Add(BRepBuilderAPI_MakeWire(circle).Wire(), face_through(block, (1, 1, 10)))
    imprint.Build()
    if not imprint.IsDone():
        raise Refused("BRepFeat_SplitShape could not imprint the circle")
    imprinted = imprint.Shape()
    disk = face_through(imprinted, (15, 10, 10))
    rest = face_through(imprinted, (1, 1, 10))
    # The disk, a face of the solid, swept 3 down from the face it lies in; fuse = 0 removes what
    # it sweeps through.
    push = BRepFeat_MakePrism(imprinted, disk, rest, gp_Dir(0, 0, -1), 0, True)
    push.Perform(3.0)
    if not push.IsDone():
        raise Refused("BRepFeat_MakePrism could not push the disk in")
    return push.Shape()
