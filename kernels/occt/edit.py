"""What the OCCT implementations of the edit cases share: editing a solid's faces directly.

Deleting faces is BRepAlgoAPI_Defeaturing: it removes the faces and heals the solid by extending
the faces round them, along their own surfaces, until they close the gap. Moving or offsetting
faces is BRepOffset_MakeOffset with no offset for the solid as a whole and an offset for each face
to move (SetOffsetOnFace). Each moves along its outward normal, and the faces beside it are
extended or trimmed to meet it (joined by intersection, so no edge is rounded). A plane moved
along its normal is the plane translated, and a cylinder offset is the cylinder about the same
axis with its radius changed.
"""
from OCP.BRep import BRep_Tool
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Defeaturing
from OCP.BRepOffset import BRepOffset_MakeOffset, BRepOffset_Skin
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType, GeomAbs_Intersection
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

TOLERANCE = 1e-7


def deleted(solid, faces):
    """`solid` with `faces` deleted and the gap they leave closed by the faces round them."""
    defeaturing = BRepAlgoAPI_Defeaturing()
    defeaturing.SetShape(solid)
    for face in faces:
        defeaturing.AddFaceToRemove(face)
    defeaturing.SetRunParallel(False)
    defeaturing.Build()
    if not defeaturing.IsDone():
        raise Refused(f"BRepAlgoAPI_Defeaturing could not delete the {len(faces)} faces")
    return defeaturing.Shape()


def moved(solid, moves):
    """`solid` with each (face, distance) of `moves` moved that far along its outward normal, and
    the faces beside it extended or trimmed to meet it."""
    offset = BRepOffset_MakeOffset()
    offset.Initialize(solid, 0.0, TOLERANCE, BRepOffset_Skin, False, False,
                      GeomAbs_Intersection, False, False)
    for face, distance in moves:
        offset.SetOffsetOnFace(face, distance)
    offset.MakeOffsetShape()
    if not offset.IsDone():
        raise Refused(f"BRepOffset_MakeOffset could not move the faces ({offset.Error().name})")
    return offset.Shape()


def drilled(solid, x, y, radius, z0, z1):
    """`solid` drilled along z by a hole of `radius` about the vertical line through (x, y), the
    cutter running from `z0` to `z1`, past the faces it comes out through."""
    cutter = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, z0), gp_Dir(0, 0, 1)),
                                      radius, z1 - z0).Shape()
    cut = BRepAlgoAPI_Cut(solid, cutter)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not drill the hole")
    return cut.Shape()


def box(lo, hi):
    """The box from the corner `lo` to the corner `hi`."""
    return BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()


def circles(solid, radius, z):
    """The edges of `solid` that are circles of `radius` in the plane at height `z`, each once."""
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and abs(curve.Circle().Radius() - radius) < TOLERANCE
                and abs(curve.Circle().Location().Z() - z) < TOLERANCE
                and not any(edge.IsSame(e) for e in found)):
            found.append(edge)
        edges.Next()
    return found


def straight_edges(solid, on):
    """The edges of `solid` both of whose ends satisfy `on`, a test of a point, each once."""
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        ends = TopExp.FirstVertex_s(edge), TopExp.LastVertex_s(edge)
        if all(on(BRep_Tool.Pnt_s(end)) for end in ends) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
        edges.Next()
    return found
