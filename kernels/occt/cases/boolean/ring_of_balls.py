"""Twelve balls on a ring, each overlapping its neighbours. Twelve balls of radius 3, their centres
evenly spaced on the circle of radius 10 about the z axis, the first at (10, 0, 0), joined one after
another."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

COUNT = 12
RING = 10
RADIUS = 3


def build():
    balls = [BRepPrimAPI_MakeSphere(gp_Pnt(RING * math.cos(2 * math.pi * i / COUNT),
                                           RING * math.sin(2 * math.pi * i / COUNT), 0),
                                    RADIUS).Shape() for i in range(COUNT)]
    ring = balls[0]
    for ball in balls[1:]:
        fuse = BRepAlgoAPI_Fuse(ring, ball)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a ball")
        ring = fuse.Shape()
    return ring
