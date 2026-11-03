"""A ball's flat deleted, closing it round. A ball of radius 10 centred on the origin, with
everything above z = 6 cut away. It has two faces: a flat disc of radius 8 in the plane z = 6, and
the rest of the sphere, which reaches its south pole, (0, 0, −10), but no longer its north.

Then edit it directly: delete the flat, and heal the solid by closing the gap it leaves with the
sphere, which runs on over the top through its north pole until it closes on itself.

The result is the whole ball of radius 10 about the origin."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from bench import Refused, face_through
from edit import box, deleted


def build():
    ball = BRepPrimAPI_MakeSphere(10).Shape()
    cut = BRepAlgoAPI_Cut(ball, box((-11, -11, 6), (11, 11, 11)))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the flat")
    truncated = cut.Shape()
    return deleted(truncated, [face_through(truncated, (0, 0, 6))])
