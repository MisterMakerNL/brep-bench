"""What the OCCT sweeps along paths with sharp corners share: a section swept along a polyline by
BRepOffsetAPI_MakePipeShell, with the corners made one of its ways.

At a corner the pipe shell either mitres the two runs (BRepBuilderAPI_RightCorner: each run's
sweep is cut by the plane bisecting the corner, and they meet in it) or turns the section about
the corner (BRepBuilderAPI_RoundCorner: the gap outside the corner is filled by the section
revolved about the axis through the corner square to both runs).
"""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon, BRepBuilderAPI_TransitionMode
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.gp import gp_Pnt

from bench import Refused

MITRE = BRepBuilderAPI_TransitionMode.BRepBuilderAPI_RightCorner
ROUND = BRepBuilderAPI_TransitionMode.BRepBuilderAPI_RoundCorner


def polyline(points, closed=False):
    poly = BRepBuilderAPI_MakePolygon()
    for point in points:
        poly.Add(gp_Pnt(*point))
    if closed:
        poly.Close()
    if not poly.IsDone():
        raise Refused("BRepBuilderAPI_MakePolygon could not make the polyline")
    return poly.Wire()


def swept(path, section, corners, binormal=None):
    """`section` (a wire at the path's start, square to it) swept along `path`, its corners made
    the `corners` way. With a `binormal`, the section keeps that direction fixed in it (a square
    keeps two sides horizontal); else the pipe shell's own frame carries it."""
    pipe = BRepOffsetAPI_MakePipeShell(path)
    if binormal is not None:
        pipe.SetMode(binormal)
    pipe.SetTransitionMode(corners)
    pipe.Add(section)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the section round the corners")
    if not pipe.MakeSolid():
        raise Refused("the swept section did not close into a solid")
    return pipe.Shape()
