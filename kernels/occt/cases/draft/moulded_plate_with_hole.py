"""A moulded plate with a bore, every wall drafted. A square plate from (−20, −20, 0) to (20, 20,
8) with a bore of radius 8 on the z axis right through it, drafted as a moulded part is: every wall
drafted 3° about the base, with one pull. The solid is pulled along +z, the neutral plane is z = 0,
and the four outer sides and the bore's wall each turn 3° about their bottom edges so that the
material narrows as it rises: the outer sides lean in, towards the axis, and the bore's wall leans
out, away from it. The base and the top stay horizontal.

So the section at height z is the 40 × 40 square moved z·tan 3° in on every side, less a disc of
radius 8 + z·tan 3°. The bore is a cone, of radius 8 at the base and 8 + 8·tan 3° ≈ 8.419 at the
top; the top is a square about 39.161 across."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from draft import drafted


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), gp_Pnt(20, 20, 8)).Shape()
    # the hole's cylinder runs past both faces of the plate
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), 8, 10).Shape()
    cut = BRepAlgoAPI_Cut(plate, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not bore the plate")
    # the same pull and angle for every wall: the outside leans in and the bore's wall out
    return drafted(cut.Shape(), 3, 0)
