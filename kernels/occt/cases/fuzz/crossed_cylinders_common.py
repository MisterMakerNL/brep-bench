"""Two cylinders crossed at any angle, their common part, twenty ways. Cylinders of radius r and
length 12r centred on the origin, along x and along (cos α, sin α, 0); their common part, turned by
ψ about the axis (θ, φ)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, turned


def cylinder(r, axis):
    """A cylinder of radius r along `axis`, from −6r to 6r."""
    start = gp_Pnt(*(-6 * r * x for x in axis))
    return BRepPrimAPI_MakeCylinder(gp_Ax2(start, gp_Dir(*axis)), r, 12 * r).Shape()


def build(p):
    r, a = p["r"], math.radians(p["alpha"])
    common = BRepAlgoAPI_Common(cylinder(r, (1, 0, 0)), cylinder(r, (math.cos(a), math.sin(a), 0)))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the crossed cylinders")
    return turned(common.Shape(), p["theta"], p["phi"], p["psi"])
