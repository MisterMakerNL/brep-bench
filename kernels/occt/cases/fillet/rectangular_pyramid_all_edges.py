"""A pyramid on a rectangle with every edge rounded, its apex split. The pyramid on the rectangle
from (-10, -6, 0) to (10, 6, 0) with its apex at (0, 0, 12), all eight edges rounded to radius 3/2:
the union of every ball of radius 3/2 inside it, the pyramid shrunk by 3/2 and grown by 3/2."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from offset import offset

BASE = [(10, -6, 0), (10, 6, 0), (-10, 6, 0), (-10, -6, 0)]
APEX = (0, 0, 12)


def pyramid():
    """The pyramid, sewn from its base and its four triangles."""
    faces = [BASE] + [[BASE[i], BASE[(i + 1) % 4], APEX] for i in range(4)]
    sewing = BRepBuilderAPI_Sewing()
    for corners in faces:
        wire = BRepBuilderAPI_MakePolygon(*[gp_Pnt(*c) for c in corners], True).Wire()
        sewing.Add(BRepBuilderAPI_MakeFace(wire).Face())
    sewing.Perform()
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(sewing.SewedShape())).Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    # Workaround: BRepFilletAPI_MakeFillet of all eight edges returns a solid BRepCheck_Analyzer
    # calls invalid, of volume 68.9, against 848.7: it has no corner for four rounds that cannot
    # meet at one ball. The solid the case asks for is the pyramid shrunk by the radius and grown
    # by it again, the union of the balls inside it, which OCCT's offsets build: the shrunk
    # pyramid's apex is a ridge, and grown, the ridge becomes the round between the long sides.
    return offset(offset(pyramid(), -1.5), 1.5)
