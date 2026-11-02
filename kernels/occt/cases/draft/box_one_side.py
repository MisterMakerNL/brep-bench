"""A box with one side drafted. The box from the origin to (20, 12, 10). Draft one of its sides, x =
20, by 10°, with the pull direction +z and the neutral plane z = 0: the side turns about its bottom
edge, the line x = 20, z = 0, until it leans 10° from the z direction, inward (toward −x) as it
rises. No other face is drafted. The top and the sides y = 0 and y = 12 are trimmed to the drafted
side.

The result is a prism along y. Its section in any plane y = const is the trapezoid through (0, 0),
(20, 0), (20 − 10·tan 10°, 10) and (0, 10), each given as (x, z), and it runs from y = 0 to y =
12."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from draft import drafted
from bench import face_through

SIZE = (20, 12, 10)
DRAFT = 10  # degrees, inward (toward −x) as the side rises


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    a, b, h = SIZE
    return drafted(box, [face_through(box, (a, b / 2, h / 2))], DRAFT)
