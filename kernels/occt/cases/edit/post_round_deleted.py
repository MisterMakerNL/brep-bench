"""A post standing out of a ball, its round deleted. The part of `fillet/ball-and-post`: a ball of
radius 10 centred on the origin, and a post, a cylinder of radius 4 on the z axis from z = 0 to z =
15, joined to it into one solid, with the circular edge where the post leaves the ball rounded to
radius 2, adding material. The round is a torus about the z axis whose centre circle has radius 6
and lies at z = 6√3; it touches the post on the circle of radius 4 at z = 6√3 and the ball on the
circle of radius 5 at z = 5√3.

Then edit the part directly: delete the round, and heal the solid by closing the gap it leaves with
the faces beside it. The ball's sphere runs on up from z = 5√3 and the post's cylinder runs on down
from z = 6√3, until they meet where they cross, in a sharp circular edge of radius 4 at z = √84.

The result is the ball and the post joined, with no round: the ball of radius 10 about the origin
together with the cylinder of radius 4 on the z axis from z = 0 to z = 15."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere

from bench import Refused, face_through
from edit import circles, deleted

BALL_RADIUS = 10
POST_RADIUS, POST_HEIGHT = 4, 15
ROUND = 2


def build():
    ball = BRepPrimAPI_MakeSphere(BALL_RADIUS).Shape()
    post = BRepPrimAPI_MakeCylinder(POST_RADIUS, POST_HEIGHT).Shape()
    part = BRepAlgoAPI_Fuse(ball, post).Shape()
    rounds = BRepFilletAPI_MakeFillet(part)
    root = math.sqrt(BALL_RADIUS ** 2 - POST_RADIUS ** 2)  # where the post leaves the ball
    for edge in circles(part, POST_RADIUS, root):
        rounds.Add(ROUND, edge)
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the post's root")
    part = rounds.Shape()

    # The round's middle: from its centre circle (radius 6 at z = 6√3), in and down at 45°.
    k = ROUND * math.sqrt(0.5)
    middle = (POST_RADIUS + ROUND - k, 0, 6 * math.sqrt(3) - k)
    return deleted(part, [face_through(part, middle)])
