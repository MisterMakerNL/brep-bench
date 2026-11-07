"""A NURBS cylinder with its top rim rounded. The cylinder of radius 5 on the z axis from z = 0 to
z = 10, its side one rational B-spline surface (the nine-point quadratic circle times a line) and
its ends flat disks, the edge where the side meets the top rounded to radius 1."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_MakeWire,
                                BRepBuilderAPI_Sewing)
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepLib import BRepLib
from OCP.Geom import Geom_BSplineSurface
from OCP.TopoDS import TopoDS
from OCP.collections import Array1_double, Array1_int, Array2_double, Array2_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pln, gp_Pnt

from bench import Refused
from edit import straight_edges

R, H = 5, 10
W = math.sqrt(2) / 2
CIRCLE = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)]
CIRCLE_W = [1, W, 1, W, 1, W, 1, W, 1]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def side():
    poles = Array2_gp_Pnt(1, len(CIRCLE), 1, 2)
    weights = Array2_double(1, len(CIRCLE), 1, 2)
    for i, ((cx, cy), w) in enumerate(zip(CIRCLE, CIRCLE_W)):
        for j, z in enumerate((0, H)):
            poles.SetValue(i + 1, j + 1, gp_Pnt(R * cx, R * cy, z))
            weights.SetValue(i + 1, j + 1, w)
    surface = Geom_BSplineSurface(poles, weights,
                                  array(Array1_double, [0, 0.25, 0.5, 0.75, 1]),
                                  array(Array1_double, [0, 1]),
                                  array(Array1_int, [3, 2, 2, 2, 3]), array(Array1_int, [2, 2]),
                                  2, 1)
    return BRepBuilderAPI_MakeFace(surface, 1e-9).Face()


def disk(z):
    rim = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), R)).Edge()
    return BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)),
                                   BRepBuilderAPI_MakeWire(rim).Wire()).Face()


def build():
    # The NURBS side and the two disks sewn into the closed cylinder.
    sewing = BRepBuilderAPI_Sewing(1e-6)
    for face in (side(), disk(0), disk(H)):
        sewing.Add(face)
    sewing.Perform()
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(sewing.SewedShape())).Solid()
    BRepLib.OrientClosedSolid_s(solid)
    fillet = BRepFilletAPI_MakeFillet(solid)
    for edge in straight_edges(solid, lambda p: abs(p.Z() - H) < 1e-9):   # the top rim
        fillet.Add(1, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the NURBS side's rim")
    return fillet.Shape()
