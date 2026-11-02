"""A pocket with its walls drafted. The block from the origin to (40, 30, 15), with a rectangular
pocket cut into its top: the box from (8, 8, 5) to (32, 22, 15), open at the block's top face, its
floor at z = 5.

Draft the pocket's four walls by 5°, with the pull direction +z and the neutral plane z = 5, the
pocket's floor: each wall turns about its bottom edge until it leans 5° from the z direction, away
from the pocket's middle as it rises, so that the pocket widens toward its opening. The pocket's
floor and the block's faces are not drafted; the floor keeps its rectangle, and the opening in the
block's top grows to meet the drafted walls.

The result is the block less a frustum of a rectangular pyramid, upside down: the pocket's section
at height z is the rectangle from (8, 8) to (32, 22) outset (z − 5)·tan 5° on every side, so its
opening is outset 10·tan 5° ≈ 0.875."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused
from draft import box_sides, drafted

BLOCK = (40, 30, 15)
POCKET = ((8, 8, 5), (32, 22, 15))
DRAFT = 5  # degrees: the walls lean away from the pocket's middle as they rise


def build():
    block = BRepPrimAPI_MakeBox(*BLOCK).Shape()
    (x0, y0, z0), (x1, y1, z1) = POCKET
    # the pocket's box runs past the top, so that the cut leaves no sliver of the top face
    pocket = BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1 + 1)).Shape()
    cut = BRepAlgoAPI_Cut(block, pocket)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    part = cut.Shape()
    # a wall facing into the pocket leans toward the block's material for a positive angle
    walls = box_sides(part, *POCKET)
    return drafted(part, walls, DRAFT, neutral=(0, 0, z0))
