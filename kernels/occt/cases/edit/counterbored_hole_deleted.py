"""A counterbored blind hole deleted. A block, the box from the origin to (40, 40, 20), with a
counterbored blind hole drilled down into its top on the vertical axis through (20, 20):

- the counterbore: radius 8, from the top, z = 20, down to its flat floor at z = 15;
- the hole: radius 4, from the counterbore's floor down to z = 5;
- the drill's point: a cone below the hole, with an included angle of 118°, its rim the hole's
  bottom circle at z = 5 and its tip on the axis 4/tan 59° ≈ 2.403 lower.

So the hole has four faces: the counterbore's wall, the ring of its floor, the hole's wall and the
point's cone. They meet edge to edge, each pair of the coaxial cylinders and cone sharing a circle.

Then edit the block directly: delete all four of the hole's faces, and heal the solid by closing the
gap they leave with the faces round them. Only the top touches them, and its plane spans the
counterbore's mouth, so it runs on across it.

The result is the undrilled block, the box from the origin to (40, 40, 20)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from edit import box, deleted

X, Y = 20, 20
BORE_RADIUS, BORE_FLOOR = 8, 15
HOLE_RADIUS, HOLE_BOTTOM = 4, 5
TIP = HOLE_RADIUS / math.tan(math.radians(118 / 2))  # the point's depth below the hole


def upright(z):
    """The vertical axis through the hole's centre, from height `z`."""
    return gp_Ax2(gp_Pnt(X, Y, z), gp_Dir(0, 0, 1))


def build():
    # The drill: the counterbore (run past the top), the hole and its point, one solid.
    bore = BRepPrimAPI_MakeCylinder(upright(BORE_FLOOR), BORE_RADIUS, 6).Shape()
    hole = BRepPrimAPI_MakeCylinder(upright(HOLE_BOTTOM), HOLE_RADIUS,
                                    BORE_FLOOR - HOLE_BOTTOM + 1).Shape()
    point = BRepPrimAPI_MakeCone(upright(HOLE_BOTTOM - TIP), 0, HOLE_RADIUS, TIP).Shape()
    drill = BRepAlgoAPI_Fuse(BRepAlgoAPI_Fuse(bore, hole).Shape(), point).Shape()
    cut = BRepAlgoAPI_Cut(box((0, 0, 0), (40, 40, 20)), drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not drill the hole")
    block = cut.Shape()

    # The hole's four faces, each by a point on it.
    faces = [face_through(block, p) for p in [
        (X + BORE_RADIUS, Y, (BORE_FLOOR + 20) / 2),  # the counterbore's wall
        (X + (BORE_RADIUS + HOLE_RADIUS) / 2, Y, BORE_FLOOR),  # its floor
        (X + HOLE_RADIUS, Y, (HOLE_BOTTOM + BORE_FLOOR) / 2),  # the hole's wall
        (X + HOLE_RADIUS / 2, Y, HOLE_BOTTOM - TIP / 2),  # the point's cone
    ]]
    return deleted(block, faces)
