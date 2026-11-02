"""A ball of radius 10 centred on the origin, and a solid cylinder of radius 5 whose axis is the
line x = 5, y = 0, parallel to the z axis, from z = −12 to z = 12. Unite them. The cylinder
touches the sphere from inside at (10, 0, 0), where Viviani's curve crosses itself."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 10
CYLINDER_RADIUS = RADIUS / 2
HALF_HEIGHT = 12

# Each seam is put where the case asks, away from the curve's crossing at (10, 0, 0): the
# sphere's poles on the y axis and its seam through (−10, 0, 0), the cylinder's seam on the z axis.
AWAY = gp_Dir(-1, 0, 0)


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Ax2(gp.Origin_s(), gp.DY_s(), AWAY), RADIUS).Shape()
    foot = gp_Pnt(CYLINDER_RADIUS, 0, -HALF_HEIGHT)
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(foot, gp.DZ_s(), AWAY), CYLINDER_RADIUS,
                                        2 * HALF_HEIGHT).Shape()
    result = BRepAlgoAPI_Fuse(ball, cylinder)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the ball and the cylinder")
    return result.Shape()
