"""A shaft's flat deleted, closing it round. A D-shaped shaft: a cylinder of radius 10 on the z
axis, from z = 0 to z = 30, with a flat along its whole length, everything with x > 7 cut away. The
flat is the plane x = 7, a strip 2√51 wide; the cylinder's side is left as one face, an arc of about
268.9° round the axis, and the ends are discs with a chord cut off.

Then edit the shaft directly: delete the flat, and heal the solid by closing the gap it leaves with
the faces round it. The cylinder's side runs on round the axis until it closes on itself, and the
two ends run on out to it, so they are whole discs again.

The result is the whole cylinder of radius 10 on the z axis from z = 0 to z = 30."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from bench import Refused, face_through
from edit import box, deleted


def build():
    cylinder = BRepPrimAPI_MakeCylinder(10, 30).Shape()
    cut = BRepAlgoAPI_Cut(cylinder, box((7, -11, -1), (11, 11, 31)))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the flat")
    shaft = cut.Shape()
    return deleted(shaft, [face_through(shaft, (7, 0, 15))])
