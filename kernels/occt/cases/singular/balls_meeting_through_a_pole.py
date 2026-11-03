"""Two balls meeting through a pole of each. Two balls of radius 5, one centred on the origin and
one on (4.8, 0, 3.6), united; each with its poles on the vertical through its centre, so the circle
where they meet passes through (0, 0, 5) and (4.8, 0, −1.4)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS = 5


def build():
    # OCCT's spheres made about a point have their poles on the vertical through it
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(4.8, 0, 3.6), RADIUS).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two balls")
    return fuse.Shape()
