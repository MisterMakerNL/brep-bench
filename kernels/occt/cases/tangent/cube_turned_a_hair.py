"""A cube united with itself turned a hair. A cube of side 10 from (−5, −5, 0) to (5, 5, 10),
united with a copy of itself turned 0.001° about the z axis."""
from math import radians

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax1, gp_Pnt, gp_Trsf

from bench import Refused

TURN = radians(0.001)


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 0), gp_Pnt(5, 5, 10)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), TURN)
    turned = BRepBuilderAPI_Transform(cube, turn, True).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, turned)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cubes")
    return fuse.Shape()
