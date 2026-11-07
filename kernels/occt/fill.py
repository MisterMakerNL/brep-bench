"""What the OCCT implementations of the fill cases share: the Coons patch of four Bézier edges,
and the solid it caps over the square from (0, 0) to (10, 10).

GeomFill_BezierCurves in GeomFill_StretchStyle makes the bilinearly blended Coons patch of four
Bézier curves that close a loop, its blending functions 1 − t and t. (GeomFill_CoonsStyle blends
with cubic Hermite functions instead, 1 − 3t² + 2t³ and 3t² − 2t³: a different surface, which for
four straight edges has the same volume under it but not the same area.) The solid's other faces are planar, each made from its own
wire, and all are sewn together (sew.py).
"""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.Geom import Geom_BezierCurve
from OCP.GeomFill import GeomFill_BezierCurves, GeomFill_FillingStyle
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt

from sew import polygon, sewn


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return Geom_BezierCurve(poles)


def capped(loop):
    """The solid over the square from (0, 0) to (10, 10), from z = 0 up to the Coons patch of the
    four Bézier curves of `loop`, each given by its control points, end to end round the square."""
    curves = [bezier(points) for points in loop]
    patch = GeomFill_BezierCurves(*curves, GeomFill_FillingStyle.GeomFill_StretchStyle).Surface()
    faces = [BRepBuilderAPI_MakeFace(patch, 1e-9).Face()]
    for points, curve in zip(loop, curves):
        start, end = points[0], points[-1]
        wire = BRepBuilderAPI_MakeWire()
        wire.Add(BRepBuilderAPI_MakeEdge(curve).Edge())
        for a, b in ((end, (end[0], end[1], 0)), ((end[0], end[1], 0), (start[0], start[1], 0)),
                     ((start[0], start[1], 0), start)):
            wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge())
        faces.append(BRepBuilderAPI_MakeFace(wire.Wire(), True).Face())
    faces.append(polygon([(0, 0, 0), (10, 0, 0), (10, 10, 0), (0, 10, 0)]))
    return sewn(faces)
