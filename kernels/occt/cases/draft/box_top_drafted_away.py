"""A box drafted until its top is gone. The box from the origin to (30, 10, 10). Draft its four
sides by 30°, with the pull direction +z and the neutral plane z = 0, the plane of its floor: each
side turns about its bottom edge until it leans 30° from the z direction, inward as it rises.

The two long sides, y = 0 and y = 10, then meet at height 5/tan 30° = 5√3 ≈ 8.660, below the top:
the draft consumes the top face. The solid is the box cut by the four drafted planes, each keeping
the side of it that holds the box's middle; the top plane, z = 10, cuts nothing off.

That is a hip roof with five faces: the floor, the rectangle from (0, 0, 0) to (30, 10, 0); two
trapezoids, from the long edges of the floor to the ridge, the line from (5, 5, 5√3) to (25, 5,
5√3); and two triangles, from the short edges of the floor to the ridge's ends."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from draft import box_sides, drafted

SIZE = (30, 10, 10)
DRAFT = 30  # degrees, inward as the sides rise: the long sides meet at 5√3, below the top


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    return drafted(box, box_sides(box, (0, 0, 0), SIZE), DRAFT)
