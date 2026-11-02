"""A box drilled across, then drafted where the hole comes out. The box from the origin to (30, 20,
20), drilled right through along x by a hole of radius 4, whose axis is the line y = 10, z = 10. The
hole comes out through the faces x = 0 and x = 30, in a circle on each.

Then draft those two faces by 8°, with the pull direction +z and the neutral plane z = 0, the plane
of the floor: each turns about its bottom edge until it leans 8° from the z direction, inward as it
rises. Nothing else is drafted. The hole's wall stays the cylinder of radius 4 about its axis,
trimmed now by the drafted faces, where it comes out in ellipses; the top and the sides y = 0 and y
= 20 are trimmed to the drafted faces.

The result is the prism along y whose section in any plane y = const is the trapezoid through (0,
0), (30, 0), (30 − 20·tan 8°, 20) and (20·tan 8°, 20), each given as (x, z), from y = 0 to y = 20,
less the cylinder of radius 4 about the hole's axis. Along its axis the hole runs from x = 10·tan 8°
to x = 30 − 10·tan 8°."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, face_through
from draft import drafted

SIZE = (30, 20, 20)
RADIUS = 4
AXIS = (10, 10)  # the hole's axis: y = 10, z = 10, along x
DRAFT = 8  # degrees, inward as the faces rise


def build():
    a, b, h = SIZE
    box = BRepPrimAPI_MakeBox(a, b, h).Shape()
    # the drill runs past both faces
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-1, *AXIS), gp.DX_s()), RADIUS, a + 2).Shape()
    cut = BRepAlgoAPI_Cut(box, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    part = cut.Shape()
    # the faces x = 0 and x = 30, by a point on each clear of the hole
    ends = [face_through(part, (x, b / 2, h / 2 + RADIUS + 2)) for x in (0, a)]
    return drafted(part, ends, DRAFT)
