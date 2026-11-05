"""A torus through a square slab, touching its four sides. A solid torus about the z axis of radii
10 and 3, united with the slab from (−13, −13, −1) to (13, 13, 1), whose sides its outer equator
touches at their middles. The torus's seam is turned 45° about z, clear of those points."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

MAJOR = 10
MINOR = 3
HALF_SIDE = 13
HALF_HEIGHT = 1


def build():
    # OCCT's torus has its seam where its frame's x direction points: here (1, 1, 0)
    torus = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s(), gp_Dir(1, 1, 0)),
                                  MAJOR, MINOR).Shape()
    slab = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, -HALF_HEIGHT),
                               gp_Pnt(HALF_SIDE, HALF_SIDE, HALF_HEIGHT)).Shape()
    fuse = BRepAlgoAPI_Fuse(torus, slab)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the torus and the slab")
    return fuse.Shape()
