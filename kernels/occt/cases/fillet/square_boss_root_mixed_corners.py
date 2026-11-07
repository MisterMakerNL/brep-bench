"""A square boss rounded at its corners and its root. The box from the origin to (40, 30, 5) joined
with the boss from (12, 7, 5) to (28, 23, 15), the boss's four upright edges rounded to radius 3,
and then its root, now a square with rounded corners, rounded to radius 2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse

from bench import Refused
from blend import edges_where, rounded
from edit import box


def upright(p):
    """On one of the boss's four upright edges, x = 12 or 28 and y = 7 or 23, above the plate."""
    return (5 < p.Z() < 15 and min(abs(p.X() - 12), abs(p.X() - 28)) < 1e-9
            and min(abs(p.Y() - 7), abs(p.Y() - 23)) < 1e-9)


def root(p):
    """On the plate's top, z = 5, within the boss's footprint's bounds."""
    return abs(p.Z() - 5) < 1e-9 and 12 - 1e-9 < p.X() < 28 + 1e-9 and 7 - 1e-9 < p.Y() < 23 + 1e-9


def build():
    fuse = BRepAlgoAPI_Fuse(box((0, 0, 0), (40, 30, 5)), box((12, 7, 5), (28, 23, 15)))
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the boss")
    plate = fuse.Shape()
    plate = rounded(plate, 3, edges_where(plate, upright), "the boss's upright edges")
    return rounded(plate, 2, edges_where(plate, root), "the boss's root")
