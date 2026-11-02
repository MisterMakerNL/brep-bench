"""A cylinder drafted both ways from a parting line. A cylinder of radius 10 about the z axis, from
z = 0 to z = 20. Split its side at the parting line, its circle at z = 10, as a part made in a
two-piece mould is, and draft each half by 3° away from the parting line, about the neutral plane z
= 10: the upper half with the pull direction +z, so that it leans inward as it rises, and the lower
half with the pull direction −z, so that it leans inward as it falls. Each half becomes a cone
through the circle at z = 10. The two ends are not drafted; each shrinks to meet its cone.

The result is two frustums of cones joined at their wide ends, with a crease between them along the
parting line: radius 10 at z = 10, and 10 − 10·tan 3° ≈ 9.476 at z = 0 and at z = 20."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused, face_through
from draft import drafted

RADIUS, HEIGHT = 10, 20
PARTING = 10  # z of the parting line
DRAFT = 3  # degrees, inward away from the parting line


def build():
    # the cylinder as its two halves glued at the parting line, so that its side is two faces
    lower = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), RADIUS, PARTING).Shape()
    upper = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, PARTING), gp.DZ_s()),
                                     RADIUS, HEIGHT - PARTING).Shape()
    fuse = BRepAlgoAPI_Fuse(lower, upper)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the two halves")
    cylinder = fuse.Shape()
    neutral = (0, 0, PARTING)
    # each half pulled away from the parting line, one draft after the other
    up = drafted(cylinder, [face_through(cylinder, (RADIUS, 0, (PARTING + HEIGHT) / 2))],
                 DRAFT, neutral=neutral, pull=(0, 0, 1))
    return drafted(up, [face_through(up, (RADIUS, 0, PARTING / 2))],
                   DRAFT, neutral=neutral, pull=(0, 0, -1))
