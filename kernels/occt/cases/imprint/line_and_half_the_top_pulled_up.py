"""A line imprinted across a top, and one side pulled up. The box from the origin to (30, 20, 10),
the line x = 12 imprinted across its top, splitting it in two; the face x ≤ 12 is then pulled up by
4, the faces beside it running on up and a wall rising along the imprinted edge."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepFeat import BRepFeat_MakePrism, BRepFeat_SplitShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Dir, gp_Pnt

from bench import Refused, face_through


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(30, 20, 10)).Shape()
    line = BRepBuilderAPI_MakeEdge(gp_Pnt(12, 0, 10), gp_Pnt(12, 20, 10)).Edge()
    imprint = BRepFeat_SplitShape(block)
    imprint.Add(line, face_through(block, (1, 1, 10)))
    imprint.Build()
    if not imprint.IsDone():
        raise Refused("BRepFeat_SplitShape could not imprint the line")
    imprinted = imprint.Shape()
    half = face_through(imprinted, (6, 10, 10))
    # The half, a face of the solid, swept 4 up from itself; fuse = 1 adds what it sweeps through.
    pull = BRepFeat_MakePrism(imprinted, half, half, gp_Dir(0, 0, 1), 1, True)
    pull.Perform(4.0)
    if not pull.IsDone():
        raise Refused("BRepFeat_MakePrism could not pull the face up")
    return pull.Shape()
