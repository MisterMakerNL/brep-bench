"""A drum under a saddle, cut level through the saddle point. The drum of geometry/saddle-drum
(radius 8, floor z = 0, top the saddle z = 10 + xy/8 made as the bilinear B-spline patch on the
corners of the square −10 ≤ x, y ≤ 10), with everything above the plane z = 10 cut away. The plane
is tangent to the saddle at (0, 0, 10) and meets it in the x and y axes raised to z = 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import (BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeHalfSpace,
                             BRepPrimAPI_MakePrism)
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from OCP.gp import gp, gp_Pln, gp_Pnt, gp_Vec

from bench import Refused

RADIUS = 8
LEVEL = 10    # the saddle point's height, and the plane's
# the saddle z = 10 + xy/8 at the corners of the square −10 ≤ x, y ≤ 10
CORNERS = [[(-10, -10, 22.5), (-10, 10, -2.5)],
           [(10, -10, -2.5), (10, 10, 22.5)]]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def saddle():
    """The bilinear B-spline patch, degree 1 each way, as a face over the whole square."""
    poles = Array2_gp_Pnt(1, 2, 1, 2)
    for i in range(2):
        for j in range(2):
            poles.SetValue(i + 1, j + 1, gp_Pnt(*CORNERS[i][j]))
    knots, mults = array(Array1_double, [0, 1]), array(Array1_int, [2, 2])
    surface = Geom_BSplineSurface(poles, knots, knots, mults, mults, 1, 1)
    return BRepBuilderAPI_MakeFace(surface, 1e-9).Face()


def build():
    # the drum as in geometry/saddle-drum: the saddle swept down past the floor, in common with
    # the drum's cylinder from the floor up past the saddle
    column = BRepPrimAPI_MakePrism(saddle(), gp_Vec(0, 0, -40)).Shape()
    drum = BRepAlgoAPI_Common(column, BRepPrimAPI_MakeCylinder(RADIUS, 30).Shape())
    if not drum.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to trim the saddle by the drum")
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, LEVEL), gp.DZ_s())).Face()
    above = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, 2 * LEVEL)).Solid()
    cut = BRepAlgoAPI_Cut(drum.Shape(), above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space above the saddle point")
    return cut.Shape()
