"""A tube bored off centre, its wall a micron thin on one side. A cylinder of radius 5 on the z axis
from z = 0 to 10, with a hole of radius 4 along the line x = 0.999999, y = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    body = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0.999999, 0, -1), gp.DZ_s()), 4, 12).Shape()
    cut = BRepAlgoAPI_Cut(body, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
