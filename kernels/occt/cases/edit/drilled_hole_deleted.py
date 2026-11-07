"""A blind drilled hole deleted, point and all. The box from the origin to (40, 30, 20) with a blind
hole along the vertical line through (20, 15): its wall the cylinder of radius 4 from the top down
to z = 10, its point the cone of half angle 45° from there to its apex at (20, 15, 6). The wall and
the point are then deleted, and the top extended along its plane to close over the hole, which
leaves the box."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from edit import box, deleted


def build():
    up = gp_Dir(0, 0, 1)
    # the drill: its body from z = 10 up past the top, and its point, a cone from radius 0 at the
    # apex to radius 4 at z = 10
    body = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(20, 15, 10), up), 4, 11).Shape()
    point = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(20, 15, 6), up), 0, 4, 4).Shape()
    drill = BRepAlgoAPI_Fuse(body, point)
    cut = BRepAlgoAPI_Cut(box((0, 0, 0), (40, 30, 20)), drill.Shape())
    if not (drill.IsDone() and cut.IsDone()):
        raise Refused("OCCT could not drill the hole")
    block = cut.Shape()
    wall = face_through(block, (24, 15, 15))
    tip = face_through(block, (22, 15, 8))
    return deleted(block, [wall, tip])
