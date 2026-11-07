"""A plate's hole wall replaced by a cone. The box from the origin to (40, 30, 8) drilled along the
line through (20, 15) by a hole of radius 5, its wall replaced by the cone about the same axis of
radius 5 at z = 0 and 7 at z = 8: a tapered hole."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from edit import box


def build():
    # Workaround: OCCT has no operation that gives a face a new surface. The same solid is the
    # plate cut by the cone's frustum, run 1 past each face: radius 4.75 at z = -1, 7.25 at z = 9.
    cutter = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(20, 15, -1), gp_Dir(0, 0, 1)), 4.75, 7.25, 10).Shape()
    cut = BRepAlgoAPI_Cut(box((0, 0, 0), (40, 30, 8)), cutter)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the tapered hole")
    return cut.Shape()
