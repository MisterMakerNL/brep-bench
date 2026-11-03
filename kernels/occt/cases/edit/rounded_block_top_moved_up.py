"""A block's top moved up, carrying its round. A block, the box from the origin to (40, 30, 10),
with the edge where its top, z = 10, meets its back, y = 30, rounded to radius 3: the round is a
quarter cylinder whose axis is the line y = 27, z = 7, tangent to the top along y = 27 and to the
back along z = 7.

Then edit the block directly: move its top face up by 6, along +z, to the plane z = 16, and carry
the round with it. The round stays a round of radius 3 tangent to the top and the back, so it moves
up by 6 too, its axis now the line y = 27, z = 13; it is not stretched, and the back and the two
ends x = 0 and x = 40 run on up to meet the moved round and top.

The result is the box from the origin to (40, 30, 16), with the edge where its top meets its back, y
= 30, rounded to radius 3."""
import math

from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet

from bench import Refused, face_through
from edit import box, deleted, moved, straight_edges

ROUND = 3
TOLERANCE = 1e-7


def rounded(block, top):
    """`block` with the edge where its top, z = `top`, meets its back, y = 30, rounded."""
    rounds = BRepFilletAPI_MakeFillet(block)
    for edge in straight_edges(block, lambda p: abs(p.Y() - 30) < TOLERANCE
                               and abs(p.Z() - top) < TOLERANCE):
        rounds.Add(ROUND, edge)
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge")
    return rounds.Shape()


def build():
    block = rounded(box((0, 0, 0), (40, 30, 10)), 10)
    # OCCT moves a face only by offsetting it, and offsetting the top alone would leave the round
    # where it is, to be stretched. So the round is deleted, the top moved, and the edge rounded
    # again where the top now meets the back: the round carried up with the top.
    k = ROUND * (1 - math.sqrt(0.5))
    sharp = deleted(block, [face_through(block, (20, 30 - k, 10 - k))])
    raised = moved(sharp, [(face_through(sharp, (20, 15, 10)), 6)])
    return rounded(raised, 16)
