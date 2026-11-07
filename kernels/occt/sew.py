"""What the OCCT implementations of the sew and fill cases share: faces made one by one, and
sewing them into the solid they bound.

Sewing is BRepBuilderAPI_Sewing: faces whose edges lie within its tolerance of each other share
those edges afterwards. The sewn shell becomes a solid with BRepBuilderAPI_MakeSolid, turned to
face outward by BRepLib::OrientClosedSolid.
"""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused


def polygon(points):
    """The planar face inside the closed polygon through `points`."""
    wire = BRepBuilderAPI_MakePolygon()
    for p in points:
        wire.Add(gp_Pnt(*p))
    wire.Close()
    return BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()


def box_faces(lo, hi, inset=0.0):
    """The six faces of the box from `lo` to `hi`, each made on its own, shrunk by `inset` on
    every side."""
    faces = []
    for axis in range(3):
        a, b = [i for i in range(3) if i != axis]
        for level in (lo[axis], hi[axis]):
            corners = []
            for sa, sb in ((0, 0), (1, 0), (1, 1), (0, 1)):
                p = [0.0, 0.0, 0.0]
                p[axis] = level
                p[a] = (lo[a] + inset, hi[a] - inset)[sa]
                p[b] = (lo[b] + inset, hi[b] - inset)[sb]
                corners.append(p)
            faces.append(polygon(corners))
    return faces


def sewn(faces, tolerance=1e-6):
    """The solid bounded by `faces` sewn into one closed shell within `tolerance`."""
    sewing = BRepBuilderAPI_Sewing(tolerance)
    for face in faces:
        sewing.Add(face)
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the faces")
    shell = TopoDS.Shell(shells.Current())
    shells.Next()
    if shells.More() or not shell.Closed():
        raise Refused("BRepBuilderAPI_Sewing did not close the faces into one shell")
    solid = BRepBuilderAPI_MakeSolid(shell).Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid
