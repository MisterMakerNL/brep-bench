"""A plate's hole deleted. The box from the origin to (40, 30, 8), drilled right through along z by
a hole of radius 5 through (20, 15); the hole's wall is then deleted, and the top and the bottom
extended along their planes to close over it, which leaves the box."""
from bench import face_through
from edit import box, deleted, drilled


def build():
    plate = drilled(box((0, 0, 0), (40, 30, 8)), 20, 15, 5, -1, 9)
    return deleted(plate, [face_through(plate, (25, 15, 4))])
