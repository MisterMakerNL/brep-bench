"""A block split by a cylindrical sheet. The box from the origin to (20, 20, 10) split by the
cylindrical sheet of radius 6 about the vertical line through (10, 10), from z = -1 to z = 11,
keeping both pieces; the post inside the sheet is then moved up by 20."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.Geom import Geom_CylindricalSurface
from OCP.gp import gp_Ax3, gp_Dir, gp_Pnt

from bench import Refused
from split import moved, split, together, volume


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 20, 10)).Shape()
    surface = Geom_CylindricalSurface(gp_Ax3(gp_Pnt(10, 10, 0), gp_Dir(0, 0, 1)), 6)
    sheet = BRepBuilderAPI_MakeFace(surface, 0, 2 * math.pi, -1, 11, 1e-7).Face()
    pieces = split(block, [sheet])
    if len(pieces) != 2:
        raise Refused(f"BRepAlgoAPI_Splitter made {len(pieces)} pieces, not 2")
    # The post's centre and the holed block's are both on the sheet's axis; the post is the
    # smaller of the two.
    post, holed = sorted(pieces, key=volume)
    return together([holed, moved(post, (0, 0, 20))])
