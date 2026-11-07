"""A helical pipe, twenty ways. A pipe of radius r along a helix of radius R about the z axis,
rising p a turn from (R, 0, 0), right- or left-handed, through n turns; its section square to the
path, its ends flat."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused


def helix(R, pitch, turns, hand):
    """The helix as a line in the (angle, height) parameters of a cylinder: angle 0 is +x, and the
    angle grows toward +y (right-handed) or toward −y (left-handed) as it rises."""
    cylinder = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), R)
    turn = 2 * math.pi if hand == "right" else -2 * math.pi
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(turn, pitch))
    length = turns * math.hypot(2 * math.pi, pitch)
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, length).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return edge


def build(p):
    path = helix(p["R"], p["p"], p["n"], p["hand"])
    start, tangent = gp_Pnt(), gp_Vec()
    curve = BRepAdaptor_Curve(path)
    curve.D1(curve.FirstParameter(), start, tangent)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), p["r"])).Edge()
    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps square to the helix
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the helix")
    if not pipe.MakeSolid():
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
