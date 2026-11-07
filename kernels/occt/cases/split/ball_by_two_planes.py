"""A ball split into three by two planes. The ball of radius 5 about the origin split at once by
the planes z = -2 and z = 2, each a square sheet wider than the ball, keeping all three pieces; the
top cap is then moved up by 10 and the bottom cap down by 10."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused
from split import centre, moved, split, together


def sheet(z):
    return BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), -10, 10, -10, 10).Face()


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    pieces = split(ball, [sheet(-2), sheet(2)])
    if len(pieces) != 3:
        raise Refused(f"BRepAlgoAPI_Splitter made {len(pieces)} pieces, not 3")
    out = []
    for piece in pieces:
        z = centre(piece)[2]
        out.append(moved(piece, (0, 0, 10)) if z > 2 else
                   moved(piece, (0, 0, -10)) if z < -2 else piece)
    return together(out)
