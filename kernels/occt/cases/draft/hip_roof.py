"""A box drafted until its top is gone: a hip roof. A box from (−10, −5, 0) to (10, 5, 10), its
four sides drafted 30° about its base: the solid is pulled along +z, the neutral plane is z = 0,
and each side turns 30° about its bottom edge so that it leans in as it rises.

The box is too tall for that draft. The section at height z is the base's 20 × 10 rectangle
moved z·tan 30° in on every side, so the two long sides (from y = ±5) meet at
z = 5/tan 30° = 5√3 ≈ 8.660, in a horizontal ridge from (−5, 0, 5√3) to (5, 0, 5√3), below the
box's top at z = 10. The top face and everything of the box above the leaning sides are gone:
the solid is the box less everything outside the four planes the drafted sides lie on. It is a
hip roof on the 20 × 10 base, its long sides two trapezoids and its short sides two triangles,
each leaning 30° from the vertical, with no top."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from draft import drafted


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(-10, -5, 0), gp_Pnt(10, 5, 10)).Shape()
    # The drafted long sides meet below the top, which the draft would have to remove.
    return drafted(box, 30, 0)
