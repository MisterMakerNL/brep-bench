"""A torus sewn from four quarters. Four faces, each the part of the torus of major radius 10 and
minor radius 3 about the z axis between two half planes through the axis a quarter turn apart,
each closed round its tube on its own seam, sewn into one closed shell and made the solid it
bounds."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.Geom import Geom_ToroidalSurface
from OCP.gp import gp_Ax3, gp_Dir, gp_Pnt

from sew import sewn


def build():
    surface = Geom_ToroidalSurface(gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), 10, 3)
    quarters = [BRepBuilderAPI_MakeFace(surface, k * math.pi / 2, (k + 1) * math.pi / 2,
                                        0, 2 * math.pi, 1e-7).Face() for k in range(4)]
    return sewn(quarters)
