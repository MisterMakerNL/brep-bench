"""A chamfer deleted where a hole comes out through it. A block, the box from the origin to (40, 20,
20), with the edge where its top, z = 20, meets its back, y = 20, chamfered 6 along both faces: the
chamfer is the plane y + z = 34, from the line y = 14 on the top to the line z = 14 on the back.

Then drill it right through along z by a hole of radius 2 whose axis is the vertical line through
(20, 17). The hole enters through the bottom in a circle and comes out through the chamfer, not the
top: across the hole, 15 ≤ y ≤ 19, the chamfer lies between z = 15 and z = 19. So the hole's wall
runs from the bottom up to an ellipse on the chamfer.

Then edit the part directly: delete the chamfer, and heal the solid by closing the gap it leaves
with the faces round it. The top and the back run on until they meet in the sharp edge y = z = 20;
the ends x = 0 and x = 40 run on to that edge too; and the hole's wall runs on up to the top, where
it now comes out in a circle.

The result is the box from the origin to (40, 20, 20), drilled right through along z by the hole of
radius 2 through (20, 17)."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer

from bench import Refused, face_through
from edit import box, deleted, drilled, straight_edges

CHAMFER = 6
TOLERANCE = 1e-7


def build():
    block = box((0, 0, 0), (40, 20, 20))
    chamfer = BRepFilletAPI_MakeChamfer(block)
    # The edge where the top, z = 20, meets the back, y = 20.
    for edge in straight_edges(block, lambda p: abs(p.Y() - 20) < TOLERANCE
                               and abs(p.Z() - 20) < TOLERANCE):
        chamfer.Add(CHAMFER, edge)
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the edge")
    part = drilled(chamfer.Shape(), 20, 17, 2, -1, 21)

    return deleted(part, [face_through(part, (5, 17, 17))])  # the chamfer, y + z = 34
