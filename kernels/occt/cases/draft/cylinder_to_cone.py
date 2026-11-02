"""A cylinder's side drafted into a cone. A cylinder of radius 10 on the z axis, from z = 0 to z =
20, its side drafted 3° about its base: the solid is pulled along +z, the neutral plane is z = 0,
and the side leans in 3° from the vertical as it rises.

So the side is a cone, and the solid a frustum: radius 10 at its base, z = 0, and
10 − 20·tan 3° ≈ 8.952 at its top, z = 20."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from draft import drafted


def build():
    cylinder = BRepPrimAPI_MakeCylinder(10, 20).Shape()
    return drafted(cylinder, 3, 0)
