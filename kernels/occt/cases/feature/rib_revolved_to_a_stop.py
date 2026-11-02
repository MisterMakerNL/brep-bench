"""A rib turned about a hub until it meets a stop. A hub with a stop, and a rib turned round the
hub until it meets the stop. Angles about the z axis are measured from +x, turning towards +y.

- The hub: a cylinder of radius 10 on the z axis, from z = 0 to z = 10.
- The stop: a wedge, everything within 25 of the z axis between the half-planes at 120° and
  135°, from z = 0 to z = 10, joined to the hub.
- The rib: the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 in the plane y = 0 (its inner part inside the
  hub), turned about the z axis from 0° towards +y until it meets the stop's face at 120°, and
  joined to them.

So the rib is the sector of an annulus from 10 to 20 out (outside the hub), 4 tall, from 0° to
120°, where its end lies flat against the stop's face, inside that face's outline; its other
end, at 0°, is a 10 × 4 rectangle in the plane y = 0. The hub, the stop and the rib are one
solid."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakeRevol
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS, TopoDS_Face
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

HUB_RADIUS, HUB_HEIGHT = 10, 10
STOP_RADIUS, STOP_FROM, STOP_TO = 25, 120, 135  # degrees
RIB = [(5, 0), (20, 0), (20, 4), (5, 4)]  # (x, z) in the plane y = 0


def direction(degrees):
    return gp_Dir(math.cos(math.radians(degrees)), math.sin(math.radians(degrees)), 0)


def stop_face(shape):
    """The stop's flat face in the half-plane at 120°, the face the rib runs up to."""
    along = direction(STOP_FROM)
    explorer = TopExp_Explorer(shape, TopAbs_FACE)
    while explorer.More():
        face = TopoDS.Face(explorer.Current())
        surface = BRepAdaptor_Surface(face)
        if surface.GetType() == GeomAbs_SurfaceType.GeomAbs_Plane:
            normal = surface.Plane().Axis().Direction()
            if abs(normal.Z()) < 1e-9 and abs(normal.Dot(along)) < 1e-9:
                return face
        explorer.Next()
    raise Refused("the stop has no face at 120°")


def build():
    hub = BRepPrimAPI_MakeCylinder(HUB_RADIUS, HUB_HEIGHT).Shape()
    stop = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), direction(STOP_FROM)),
                                    STOP_RADIUS, HUB_HEIGHT,
                                    math.radians(STOP_TO - STOP_FROM)).Shape()
    base = BRepAlgoAPI_Fuse(hub, stop)
    if not base.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the stop to the hub")
    base = base.Shape()
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in RIB:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    profile = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    # A boss (fuse) turned about the z axis, from +x towards +y, until it meets the stop.
    rib = BRepFeat_MakeRevol(base, profile, TopoDS_Face(), gp.OZ_s(), 1, True)
    rib.Perform(stop_face(base))
    if not rib.IsDone():
        raise Refused(f"BRepFeat_MakeRevol could not turn the rib: {rib.CurrentStatusError()}")
    return rib.Shape()
