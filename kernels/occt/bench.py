"""What the OCCT implementations of brep-bench share: the one way a case says it cannot be built.

A case's module defines `build()`, which returns the solid as a `TopoDS_Shape`, or raises
`Refused` when OCCT declines an operation (a builder that is not done) or the case asks for
something OCCT cannot do. `run.py` measures what comes back. A face is picked by a point on it,
as a description names a face by where it is (`face_through`).
"""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex, BRepBuilderAPI_Transform
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf

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


def direction(theta, phi):
    """The unit direction (sin θ cos φ, sin θ sin φ, cos θ), θ and φ in degrees, as a family's
    description names an axis."""
    t, f = math.radians(theta), math.radians(phi)
    return (math.sin(t) * math.cos(f), math.sin(t) * math.sin(f), math.cos(t))


def turned(shape, theta, phi, psi):
    """`shape` turned by ψ degrees about the axis through the origin with direction (θ, φ)."""
    trsf = gp_Trsf()
    trsf.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(*direction(theta, phi))), math.radians(psi))
    return BRepBuilderAPI_Transform(shape, trsf, True).Shape()
