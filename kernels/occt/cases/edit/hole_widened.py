"""A plate's hole widened. A plate, the box from the origin to (40, 30, 8), drilled right through
along z by a hole of radius 5 whose axis is the vertical line through (20, 15).

Then edit it directly: offset the hole's wall by 2 into the plate, away from its axis, so that it
becomes the cylinder of radius 7 about the same axis. The top and the bottom are trimmed back to
meet it.

The result is the box from the origin to (40, 30, 8), drilled right through along z by a hole of
radius 7 through (20, 15)."""
from bench import face_through
from edit import box, drilled, moved


def build():
    plate = drilled(box((0, 0, 0), (40, 30, 8)), 20, 15, 5, -1, 9)
    wall = face_through(plate, (25, 15, 4))
    # The wall's outward normal points out of the plate, into the hole, toward its axis: into
    # the plate, away from the axis, is the other way.
    return moved(plate, [(wall, -2)])
