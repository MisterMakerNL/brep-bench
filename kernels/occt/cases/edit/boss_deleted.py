"""A plate's boss deleted. The box from the origin to (40, 30, 5) with the cylinder of radius 6
about the vertical line through (20, 15), from z = 5 to z = 15, joined to its top; the boss's side
and top are then deleted, and the plate's top extended along its plane to close over the boss's
footprint, which leaves the plate."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from edit import box, deleted


def build():
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(20, 15, 5), gp_Dir(0, 0, 1)), 6, 10).Shape()
    fuse = BRepAlgoAPI_Fuse(box((0, 0, 0), (40, 30, 5)), boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the boss")
    plate = fuse.Shape()
    side = face_through(plate, (26, 15, 10))
    top = face_through(plate, (20, 15, 15))
    return deleted(plate, [side, top])
