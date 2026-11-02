"""A pocket with rounded corners and its walls drafted. The block from the origin to (40, 30, 15),
with a pocket cut into its top: the box from (8, 8, 5) to (32, 22, 15), its four vertical edges
rounded to radius 4, open at the block's top face, its floor at z = 5. The pocket's walls are four
planes and four quarter cylinders of radius 4 about the vertical lines through (12, 12), (28, 12),
(28, 18) and (12, 18), each tangent to the planes beside it: concave rounds, seen from the block's
material.

Draft all eight walls by 5°, with the pull direction +z and the neutral plane z = 5, the pocket's
floor, so that each leans 5° from the z direction, away from the pocket's middle as it rises. Each
plane turns about its bottom edge; each quarter cylinder becomes a quarter cone about the same axis,
its radius 4 + (z − 5)·tan 5° at height z, so that it stays tangent to the planes beside it. The
pocket's floor and the block's faces are not drafted; the opening in the block's top grows to meet
the drafted walls.

The result is the block less the pocket, whose section at height z is its floor's outline, the
rounded rectangle, moved (z − 5)·tan 5° outward: its straight sides outset by that and its corners'
radii 4 + (z − 5)·tan 5° about the same centres."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from draft import box_sides, drafted, rounded_box

BLOCK = (40, 30, 15)
POCKET = ((8, 8, 5), (32, 22, 15))
RADIUS = 4
DRAFT = 5  # degrees: the walls lean away from the pocket's middle as they rise


def build():
    block = BRepPrimAPI_MakeBox(*BLOCK).Shape()
    (x0, y0, z0), (x1, y1, z1) = POCKET
    # the pocket's box runs past the top, so that the cut leaves no sliver of the top face
    pocket = rounded_box((x0, y0, z0), (x1, y1, z1 + 1), RADIUS)
    cut = BRepAlgoAPI_Cut(block, pocket)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    part = cut.Shape()
    # a wall facing into the pocket leans toward the block's material for a positive angle
    walls = box_sides(part, *POCKET, RADIUS)
    return drafted(part, walls, DRAFT, neutral=(0, 0, z0))
