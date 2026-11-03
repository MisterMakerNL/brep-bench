"""A plate's hole deleted. A plate, the box from the origin to (40, 30, 8), drilled right through
along z by a hole of radius 5 whose axis is the vertical line through (20, 15). The hole's wall is
one cylindrical face, and the plate's top and bottom each have a circular edge round it.

Then edit the drilled plate directly: delete the hole's wall, and heal the solid by closing the gap
it leaves with the faces round it. The top and bottom are planes that already span the hole, so each
runs on across it and its circular edge goes.

The result is the undrilled plate, the box from the origin to (40, 30, 8)."""
from bench import face_through
from edit import box, deleted, drilled


def build():
    plate = drilled(box((0, 0, 0), (40, 30, 8)), 20, 15, 5, -1, 9)
    wall = face_through(plate, (25, 15, 4))
    return deleted(plate, [wall])
