"""A wedge a millionth steep. A cube of side 10 from (−5, −5, −10) to (5, 5, 0), in common with the
half-space above the plane z = y·10⁻⁶, which crosses the top face along its middle line, the x
axis: a wedge 5·10⁻⁶ thick at y = −5, thinning to an edge at y = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeHalfSpace
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

HALF = 5
SLOPE = 1e-6


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, -2 * HALF), gp_Pnt(HALF, HALF, 0)).Shape()
    # the plane through the top face's middle line, its normal (0, −SLOPE, 1)
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp_Dir(0, -SLOPE, 1))).Face()
    above = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, 2 * HALF)).Solid()
    common = BRepAlgoAPI_Common(cube, above)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the half-space")
    return common.Shape()
