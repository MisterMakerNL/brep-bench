"""A ball's cap a ten-thousandth high, around its pole. A ball of radius 5 centered on the origin,
in common with the half-space above the plane z = 4.9999: the cap 10⁻⁴ high, its rim a circle of
radius about 0.0316 around the pole at (0, 0, 5)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5
HEIGHT = 1e-4  # of the cap


def build():
    # OCCT's sphere has its poles on the z axis of its placement: here the global z axis
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, RADIUS - HEIGHT), gp.DZ_s())).Face()
    above = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, 2 * RADIUS)).Solid()
    common = BRepAlgoAPI_Common(ball, above)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the ball and the half-space")
    return common.Shape()
