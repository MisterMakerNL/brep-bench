"""A box with its four sides drafted. A box from (−20, −15, 0) to (20, 15, 10), its four sides
drafted 5° about its base: the solid is pulled along +z, the neutral plane is z = 0, and each side
turns 5° about its bottom edge so that it leans in as it rises. The base and the top stay
horizontal.

So the section at height z is the base's 40 × 30 rectangle moved z·tan 5° in on every side, and
the top is a rectangle (40 − 20·tan 5°) × (30 − 20·tan 5°), about 38.250 × 28.250. The sides are
four trapezoids, each leaning 5° from the vertical."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from draft import drafted


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(-20, -15, 0), gp_Pnt(20, 15, 10)).Shape()
    return drafted(box, 5, 0)
