"""A cylinder sewn from its side and two disks. The cylindrical face of radius 5 about the z axis
from z = 0 to z = 10, closed on its seam in the half plane y = 0, x ≥ 0, and the disks of radius 5
on z = 0 and z = 10, each made on its own, sewn into one closed shell and made the solid it
bounds."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.Geom import Geom_CylindricalSurface
from OCP.gp import gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Pln, gp_Pnt

from sew import sewn

R, H = 5, 10


def disk(z):
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), R)).Edge()
    wire = BRepBuilderAPI_MakeWire(circle).Wire()
    return BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), wire).Face()


def build():
    # The surface's u = 0 is its x direction, so its seam lies in the half plane y = 0, x ≥ 0.
    surface = Geom_CylindricalSurface(gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), R)
    side = BRepBuilderAPI_MakeFace(surface, 0, 2 * math.pi, 0, H, 1e-7).Face()
    return sewn([side, disk(0), disk(H)])
