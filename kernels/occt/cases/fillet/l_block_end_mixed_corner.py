"""An L-block's end rounded across its inside corner. The union of the boxes from the origin to
(20, 10, 6) and to (6, 10, 20), its concave edge from (6, 0, 6) to (6, 10, 6) rounded to radius 2,
and then the end face's edge chain, up the upright, round the concave round's end and along the
foot, rounded to radius 2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse

from bench import Refused
from blend import edges_where, rounded
from edit import box


def on_end_chain(p):
    """On the end face y = 0, and on the upright's face x = 6, the foot's top z = 6, or the
    concave round's quarter circle about (8, 0, 8)."""
    return abs(p.Y()) < 1e-9 and (
        (abs(p.X() - 6) < 1e-9 and p.Z() > 8 - 1e-9)
        or (abs(p.Z() - 6) < 1e-9 and p.X() > 8 - 1e-9)
        or (abs((p.X() - 8) ** 2 + (p.Z() - 8) ** 2 - 4) < 1e-6 and p.X() < 8 and p.Z() < 8))


def build():
    fuse = BRepAlgoAPI_Fuse(box((0, 0, 0), (20, 10, 6)), box((0, 0, 0), (6, 10, 20)))
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the foot and the upright")
    block = fuse.Shape()
    concave = edges_where(block, lambda p: abs(p.X() - 6) < 1e-9 and abs(p.Z() - 6) < 1e-9)
    block = rounded(block, 2, concave, "the concave edge")
    # One after another, as the case composes the corner. Given all three edges at once,
    # BRepFilletAPI_MakeFillet closes the corner its own way: volume 2024.2144, against 2024.6832.
    return rounded(block, 2, edges_where(block, on_end_chain), "the end face's edge chain")
