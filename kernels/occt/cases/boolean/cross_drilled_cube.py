"""A cube drilled through on a grid three ways. A cube from the origin to (30, 30, 30), drilled by
fifteen holes of radius 2.5: three along x at y = 7.5, 15, 22.5 and z = 15, three along y at
x = 7.5, 15, 22.5 and z = 15, and nine along z through (x, y) for x and y each 7.5, 15, 22.5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

SIDE = 30
RADIUS = 2.5
GRID = (7.5, 15, 22.5)
MIDDLE = 15


def drill(start, axis):
    """A drill from `start` along `axis`, running a unit past the cube at both ends."""
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*start), axis), RADIUS, SIDE + 2).Shape()


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    drills = ([drill((-1, g, MIDDLE), gp.DX_s()) for g in GRID]
              + [drill((g, -1, MIDDLE), gp.DY_s()) for g in GRID]
              + [drill((x, y, -1), gp.DZ_s()) for x in GRID for y in GRID])
    for tool in drills:
        cut = BRepAlgoAPI_Cut(cube, tool)
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a hole")
        cube = cut.Shape()
    return cube
