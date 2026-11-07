"""The lens common to two NURBS balls. Balls of radius 5 about (-3, 0, 0) and (3, 0, 0), each
bounded by one standard biquadratic NURBS sphere (geometry/nurbs-sphere) moved to its centre, and
their common part, a lens."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.gp import gp_Trsf, gp_Vec

from bench import Refused, other_case


def ball(x):
    trsf = gp_Trsf()
    trsf.SetTranslation(gp_Vec(x, 0, 0))
    return BRepBuilderAPI_Transform(other_case("geometry/nurbs-sphere").build(), trsf, True).Shape()


def build():
    common = BRepAlgoAPI_Common(ball(-3), ball(3))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not intersect the two NURBS balls")
    return common.Shape()
