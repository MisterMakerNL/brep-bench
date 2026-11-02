"""A box with one side drafted. A box from (0, 0, 0) to (30, 20, 10) with one side drafted: the
face x = 30 turns 10° about its bottom edge (the solid is pulled along +z, the neutral plane is z =
0) so that it leans in as it rises, meeting the top at x = 30 − 10·tan 10° ≈ 28.237. No other face
is drafted.

The faces beside it, y = 0 and y = 20, stay where they are and are trimmed by the leaning face
into trapezoids; the top shrinks to 28.237 × 20; the base and the face x = 0 are unchanged."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.gp import gp_Pnt

from draft import drafted


def is_end(face):
    """The face x = 30."""
    surface = BRepAdaptor_Surface(face)
    return (surface.GetType() == GeomAbs_SurfaceType.GeomAbs_Plane
            and abs(surface.Plane().Location().X() - 30) < 1e-9
            and abs(abs(surface.Plane().Axis().Direction().X()) - 1) < 1e-9)


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(30, 20, 10)).Shape()
    return drafted(box, 10, 0, chosen=is_end)
