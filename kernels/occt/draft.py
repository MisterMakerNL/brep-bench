"""What the OCCT draft cases share: drafting chosen faces of a solid with BRepOffsetAPI_DraftAngle.

A draft turns each chosen face about its line on a neutral plane, by an angle, so that it leans
from the direction the solid is pulled in: a side of a box becomes a sloping plane, a cylinder's
side a cone. The faces next to it are trimmed or extended to meet it.
"""
import math

from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

TOLERANCE = 1e-9


def faces(shape):
    explorer = TopExp_Explorer(shape, TopAbs_FACE)
    while explorer.More():
        yield TopoDS.Face(explorer.Current())
        explorer.Next()


def is_cap(face, pull):
    """A plane face square to the pull: the draft leaves it where it is."""
    surface = BRepAdaptor_Surface(face)
    if surface.GetType() != GeomAbs_SurfaceType.GeomAbs_Plane:
        return False
    return abs(abs(surface.Plane().Axis().Direction().Dot(pull)) - 1) < TOLERANCE


def drafted(shape, degrees, neutral_z, chosen=None):
    """`shape` with its faces drafted by `degrees` about the plane z = `neutral_z`, pulled along +z,
    so that each leans in as z grows. The faces drafted are those `chosen` picks (by default
    every face that is not a cap square to z)."""
    pull = gp_Dir(0, 0, 1)
    neutral = gp_Pln(gp_Pnt(0, 0, neutral_z), pull)
    draft = BRepOffsetAPI_DraftAngle(shape)
    for face in faces(shape):
        if (chosen or (lambda f: not is_cap(f, pull)))(face):
            draft.Add(face, pull, math.radians(degrees), neutral)
            if not draft.AddDone():
                raise Refused("BRepOffsetAPI_DraftAngle declined a face")
    draft.Build()
    if not draft.IsDone():
        raise Refused(f"BRepOffsetAPI_DraftAngle could not draft the faces: {draft.Status()}")
    return draft.Shape()
