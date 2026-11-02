"""A box with its four sides drafted outward. The box from the origin to (20, 12, 10). Draft its
four sides by 5° the other way, with the pull direction +z and the neutral plane z = 0, the plane of
its floor: each side turns about the line where it meets z = 0 until it leans 5° from the z
direction, outward as it rises (a negative draft, as on the walls of a part that must come out of
its mould downward). The floor and the top are not drafted; the top grows to meet the drafted sides.

The result is an upside-down frustum of a rectangular pyramid, with six faces. Its floor is the
rectangle from (0, 0, 0) to (20, 12, 0), and its section at height z is that rectangle outset z·tan
5° on every side, so its top is outset 10·tan 5° ≈ 0.875."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from draft import box_sides, drafted

SIZE = (20, 12, 10)
DRAFT = -5  # degrees, negative: the sides lean outward as they rise


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    return drafted(box, box_sides(box, (0, 0, 0), SIZE), DRAFT)
