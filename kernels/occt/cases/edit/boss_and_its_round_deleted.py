"""A boss and the round at its base deleted. A plate, the box from the origin to (60, 40, 6), with a
boss on its top: a cylinder of radius 8 on the vertical axis through (30, 20), from z = 6 to z = 16,
joined to the plate. The concave circular edge where the boss stands on the plate is rounded to
radius 2, adding material: the round is a quarter torus whose centre circle has radius 10 and lies
at z = 8, tangent to the plate's top on the circle of radius 10 and to the boss's side at z = 8.

Then edit the part directly: delete the boss's three faces, its flat top, its side and the round at
its base, and heal the solid by closing the gap they leave with the faces round them. Only the
plate's top touches them, and its plane spans the round's outer circle, so it runs on across it.

The result is the plate alone, the box from the origin to (60, 40, 6)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from edit import box, circles, deleted

X, Y = 30, 20
BOSS_RADIUS, PLATE_TOP, BOSS_TOP = 8, 6, 16
ROUND = 2


def build():
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(X, Y, PLATE_TOP), gp_Dir(0, 0, 1)),
                                    BOSS_RADIUS, BOSS_TOP - PLATE_TOP).Shape()
    part = BRepAlgoAPI_Fuse(box((0, 0, 0), (60, 40, PLATE_TOP)), boss).Shape()
    rounds = BRepFilletAPI_MakeFillet(part)
    for edge in circles(part, BOSS_RADIUS, PLATE_TOP):  # where the boss stands on the plate
        rounds.Add(ROUND, edge)
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the boss's base")
    part = rounds.Shape()

    k = ROUND * (1 - math.sqrt(0.5))  # the round's middle, out from the boss and up the plate
    boss_faces = [face_through(part, p) for p in [
        (X, Y, BOSS_TOP),  # its top
        (X + BOSS_RADIUS, Y, (PLATE_TOP + BOSS_TOP) / 2),  # its side
        (X + BOSS_RADIUS + k, Y, PLATE_TOP + k),  # the round at its base
    ]]
    return deleted(part, boss_faces)
