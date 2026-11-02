"""A hole drafted into a taper. The block from (−15, −15, 0) to (15, 15, 20), bored right through
along the z axis by a hole of radius 6.

Draft the hole's wall by 3°, with the pull direction +z and the neutral plane z = 0, the block's
floor: the wall becomes a cone through the hole's circle of radius 6 at z = 0, leaning 3° from the z
direction, away from the axis as it rises, so that the hole widens toward the top. The block's faces
are not drafted; the hole's circle in the top face grows to meet the cone.

The result is the block less a frustum of a cone: radius 6 at z = 0 and 6 + 20·tan 3° ≈ 7.048 at z =
20."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, face_through
from draft import drafted

HALF, HEIGHT = 15, 20
RADIUS = 6
DRAFT = 3  # degrees: the wall leans away from the axis as it rises


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, 0), gp_Pnt(HALF, HALF, HEIGHT)).Shape()
    # the drill runs past both faces
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()),
                                     RADIUS, HEIGHT + 2).Shape()
    cut = BRepAlgoAPI_Cut(block, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    part = cut.Shape()
    return drafted(part, [face_through(part, (RADIUS, 0, HEIGHT / 2))], DRAFT)
