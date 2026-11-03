"""A drilled block's top moved up. A block, the box from the origin to (40, 30, 10), drilled right
through along z by a hole of radius 5 whose axis is the vertical line through (20, 15).

Then edit it directly: move its top face, the plane z = 10 with the hole's circle in it, up by 6,
along +z, to the plane z = 16. The faces beside the top follow it: the four sides and the hole's
wall run on up to meet it there.

The result is the box from the origin to (40, 30, 16), drilled right through along z by the hole of
radius 5 through (20, 15)."""
from bench import face_through
from edit import box, drilled, moved


def build():
    block = drilled(box((0, 0, 0), (40, 30, 10)), 20, 15, 5, -1, 11)
    top = face_through(block, (5, 5, 10))
    return moved(block, [(top, 6)])  # its outward normal is +z
