"""A cylinder's rounded rim made sharp again. A cylinder of radius 5 on the z axis, from z = 0 to z
= 10, with the circular edge round its top face rounded to radius 1: the round is a quarter torus
whose centre circle has radius 4 and lies at z = 9, tangent to the top on the circle of radius 4 and
to the side at z = 9.

Then edit it directly: delete the round, and heal the solid by closing the gap it leaves with the
faces beside it. The side, a cylinder, runs on up to z = 10, and the top, a plane, runs on out to
radius 5, where they meet in a sharp circular edge.

The result is the cylinder of radius 5 on the z axis from z = 0 to z = 10, with no round."""
import math

from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from bench import Refused, face_through
from edit import circles, deleted

RADIUS, HEIGHT = 5, 10
ROUND = 1


def build():
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    rounds = BRepFilletAPI_MakeFillet(cylinder)
    for edge in circles(cylinder, RADIUS, HEIGHT):
        rounds.Add(ROUND, edge)
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the rim")
    rounded = rounds.Shape()

    k = ROUND * (1 - math.sqrt(0.5))  # the round's middle, in from the side and down from the top
    return deleted(rounded, [face_through(rounded, (RADIUS - k, 0, HEIGHT - k))])
