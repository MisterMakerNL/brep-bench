"""A box's top replaced by a cylinder. The box from the origin to (20, 10, 10), its top's plane
replaced by the cylinder of radius 10√2 whose axis is the line x = 10, z = 0, along y, the sides
y = 0 and y = 10 extended up to meet it: the region over the rectangle from (0, 0) to (20, 10) from
z = 0 up to the cylinder."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from edit import box


def build():
    # Workaround: OCCT can move and delete a solid's faces, but has no operation that gives a face
    # a new surface. The same solid is a box tall enough to hold the arch, less what lies outside
    # the cylinder: their common part.
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, -1, 0), gp_Dir(0, 1, 0)),
                                        10 * math.sqrt(2), 12).Shape()
    common = BRepAlgoAPI_Common(box((0, 0, 0), (20, 10, 20)), cylinder)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not trim the box to the cylinder")
    return common.Shape()
