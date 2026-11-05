"""A cylinder inscribed through a cube. A cube of side 10 centered on the origin, united with a
cylinder of radius 5 on the x axis from x = −10 to x = 10, whose wall touches four of the cube's
faces along their middle lines. The cylinder's seam is put along the line y = z of its wall."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

HALF = 5
RADIUS = 5
REACH = 10    # the cylinder runs from x = −REACH to x = REACH


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, -HALF), gp_Pnt(HALF, HALF, HALF)).Shape()
    # OCCT's cylinder has its seam along its frame's x direction: here (0, 1, 1), at 45° to the
    # lines where the wall touches the cube
    frame = gp_Ax2(gp_Pnt(-REACH, 0, 0), gp.DX_s(), gp_Dir(0, 1, 1))
    cylinder = BRepPrimAPI_MakeCylinder(frame, RADIUS, 2 * REACH).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, cylinder)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the cylinder")
    return fuse.Shape()
