"""What the OCCT implementations of the draft cases share: drafting faces of a solid with
BRepOffsetAPI_DraftAngle.

DraftAngle turns each face it is given about its neutral curve, where the face crosses the neutral
plane, until it makes the given angle with the pull direction: a plane turns about a line, and a
cylinder whose axis is the pull direction becomes a cone through the same circle. A positive angle
leans the face toward the solid's inside as it runs along the pull direction (an outer side
narrows, a hole's wall widens), and a negative one away from it. The faces beside the drafted ones
are extended or trimmed to meet them.
"""
import math

from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused


def drafted(solid, faces, degrees, neutral=(0, 0, 0), pull=(0, 0, 1)):
    """`solid` with each of `faces` drafted `degrees` about the plane through `neutral` square to
    `pull`. Refused if OCCT declines a face or the whole."""
    direction = gp_Dir(*pull)
    plane = gp_Pln(gp_Pnt(*neutral), direction)
    draft = BRepOffsetAPI_DraftAngle(solid)
    for face in faces:
        draft.Add(face, direction, math.radians(degrees), plane)
        if not draft.AddDone():
            raise Refused("BRepOffsetAPI_DraftAngle declined a face: the draft would leave it no "
                          "valid neighbours")
    draft.Build()
    if not draft.IsDone():
        # Draft_EdgeRecomputation and the like: which part of the solid it could not recompute
        status = draft.Status().name.removeprefix("Draft_")
        raise Refused(f"BRepOffsetAPI_DraftAngle could not build the drafted solid ({status})")
    return draft.Shape()
