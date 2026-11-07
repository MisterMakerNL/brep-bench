"""A block's round deleted. The box from the origin to (20, 10, 10) with the edge where its top
meets its end x = 20 rounded to radius 3; the round's face is then deleted, and the faces around it
extended along their own surfaces to close the gap, which leaves the box."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet

from bench import Refused, face_through
from edit import box, deleted, straight_edges


def build():
    block = box((0, 0, 0), (20, 10, 10))
    fillet = BRepFilletAPI_MakeFillet(block)
    for edge in straight_edges(block, lambda p: abs(p.X() - 20) < 1e-9 and abs(p.Z() - 10) < 1e-9):
        fillet.Add(3, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge")
    rounded = fillet.Shape()
    # a point on the round, halfway round its quarter circle about the axis x = 17, z = 7
    round_face = face_through(rounded, (17 + 3 * 0.5 ** 0.5, 5, 7 + 3 * 0.5 ** 0.5))
    return deleted(rounded, [round_face])
