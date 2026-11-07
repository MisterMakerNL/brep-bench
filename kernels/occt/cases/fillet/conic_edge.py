"""A block's edge blended by a parabolic conic. The box from the origin to (20, 10, 10), the edge
where its top meets its end x = 20 blended by a conic of rho 0.5, a parabola tangent to the top and
the end each 3 from the edge: in every plane y = c, the quadratic Bézier curve through (17, c, 10)
and (20, c, 7) whose middle control point is the edge's point (20, c, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused


def build():
    # Workaround: OCCT's fillets have a circular section (ChFi3d's other fillet shapes only
    # approximate the circle), and it has no conic blend. The same solid is the block less the
    # corner outside the parabola: the parabola, closed round the corner outside the block, swept
    # along y past both ends.
    y = -1
    poles = Array1_gp_Pnt(1, 3)
    for i, p in enumerate([(17, y, 10), (20, y, 10), (20, y, 7)]):
        poles.SetValue(i + 1, gp_Pnt(*p))
    parabola = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(parabola)
    corners = [(20, y, 7), (21, y, 7), (21, y, 11), (17, y, 11), (17, y, 10)]
    for a, b in zip(corners, corners[1:]):
        wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge())
    corner = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire(), True).Face(),
                                   gp_Vec(0, 12, 0)).Shape()
    cut = BRepAlgoAPI_Cut(BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 10, 10)).Shape(), corner)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the corner away")
    return cut.Shape()
