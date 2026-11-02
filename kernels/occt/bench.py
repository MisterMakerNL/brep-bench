"""What the OCCT implementations of brep-bench share: the one way a case says it cannot be built.

A case's module defines `build()`, which returns the solid as a `TopoDS_Shape`, or raises
`Refused` when OCCT declines an operation (a builder that is not done) or the case asks for
something OCCT cannot do. `run.py` measures what comes back. A face is picked by a point on it,
as a description names a face by where it is (`face_through`).
"""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

# How near a point must be to a face to pick it.
FACE_TOLERANCE = 1e-7


class Refused(Exception):
    """OCCT cannot build this case, or one of its operations declined; the message says which."""


def face_through(solid, point):
    """The one face of `solid` that `point` lies on."""
    vertex = BRepBuilderAPI_MakeVertex(gp_Pnt(*point)).Vertex()
    found = []
    faces = TopExp_Explorer(solid, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        if (BRepExtrema_DistShapeShape(vertex, face).Value() < FACE_TOLERANCE
                and not any(face.IsSame(f) for f in found)):
            found.append(face)
        faces.Next()
    assert len(found) == 1, f"{len(found)} faces through {point}"
    return found[0]
