"""Two half tori of one torus, overlapping. The halves z ≥ 0 and x ≥ 0 of the solid torus about z
of radii 10 and 3, united; the second's seam turned a quarter turn about z from the first's."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

MAJOR = 10
MINOR = 3


def half(seam, keep):
    """The half of the torus on the + side of the plane through the origin square to `keep`, the
    torus's seam where `seam` points."""
    # OCCT's torus has its seam where its frame's x direction points
    torus = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s(), gp_Dir(*seam)),
                                  MAJOR, MINOR).Shape()
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp_Dir(*keep))).Face()
    behind = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(*(-c for c in keep))).Solid()
    cut = BRepAlgoAPI_Cut(torus, behind)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined to halve the torus")
    return cut.Shape()


def build():
    fuse = BRepAlgoAPI_Fuse(half((1, 0, 0), (0, 0, 1)), half((0, 1, 0), (1, 0, 0)))
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two halves")
    return fuse.Shape()
