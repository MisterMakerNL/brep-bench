"""A box drafted about its middle. The box from the origin to (20, 12, 10). Draft its four sides by
5°, with the pull direction +z and the neutral plane z = 5, halfway up: each side turns about the
line where it crosses z = 5 until it leans 5° from the z direction, inward as it rises. So below z =
5 each side moves out, and above it each side moves in. The floor and the top are not drafted: the
floor grows to meet the drafted sides, and the top shrinks to meet them.

The result is a frustum of a rectangular pyramid, with six faces. Its section at height z is the
box's own rectangle, from (0, 0) to (20, 12), inset (z − 5)·tan 5° on every side: outset 5·tan 5° ≈
0.437 at the floor, the box's own at z = 5, and inset as much at the top."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from draft import drafted
from shell import face_through

SIZE = (20, 12, 10)
DRAFT = 5  # degrees, inward as the sides rise
NEUTRAL = (0, 0, 5)  # halfway up

def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    a, b, h = SIZE
    sides = [face_through(box, p) for p in
             [(0, b / 2, h / 2), (a, b / 2, h / 2), (a / 2, 0, h / 2), (a / 2, b, h / 2)]]
    return drafted(box, sides, DRAFT, neutral=NEUTRAL)
