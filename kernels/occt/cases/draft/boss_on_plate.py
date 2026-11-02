"""A boss on a plate, its side drafted. A square plate from (−20, −20, 0) to (20, 20, 5), and on it
a cylindrical boss of radius 8 about the z axis, from z = 5 to z = 17, united with the plate.

Draft the boss's side by 4°, with the pull direction +z and the neutral plane z = 5, the plate's top
face, where the boss stands: the side becomes a cone through the boss's circle of radius 8 at z = 5,
leaning 4° from the z direction, inward as it rises. The plate and the boss's top are not drafted;
the top shrinks to meet the cone.

The result is the plate with a frustum of a cone on it: radius 8 at z = 5 and 8 − 12·tan 4° ≈ 7.161
at its top, z = 17."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from draft import drafted
from shell import face_through

HALF, THICKNESS = 20, 5
BOSS_RADIUS, BOSS_HEIGHT = 8, 12
DRAFT = 4  # degrees, inward as the boss rises


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, 0), gp_Pnt(HALF, HALF, THICKNESS)).Shape()
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, THICKNESS), gp.DZ_s()),
                                    BOSS_RADIUS, BOSS_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    part = fuse.Shape()
    side = face_through(part, (BOSS_RADIUS, 0, THICKNESS + BOSS_HEIGHT / 2))
    # about the plate's top, where the boss stands
    return drafted(part, [side], DRAFT, neutral=(0, 0, THICKNESS))
