"""A hole drilled through the upper arm only. A C-shaped bracket with a hole drilled through its
upper arm and no further.

The bracket: the polygon through (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24), (40, 24),
(40, 30), (0, 30) in the plane y = −10, extruded along +y to y = 10. So it has a lower arm from
z = 0 to 6 and an upper arm from z = 24 to 30, both 40 long, joined by a back from x = 0 to 6,
with a gap 18 tall between the arms.

The hole: radius 4, on the vertical axis through (28, 0), drilled down from the upper arm's top
through the first wall it meets: through the upper arm, from z = 30 to z = 24, ending where it
comes out into the gap. The lower arm is not drilled."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakeCylindricalHole, BRepFeat_Status
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

SECTION = [(0, 0), (40, 0), (40, 6), (6, 6), (6, 24), (40, 24), (40, 30), (0, 30)]  # (x, z)
RADIUS = 4


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in SECTION:
        outline.Add(gp_Pnt(x, -10, z))
    outline.Close()
    bracket = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(outline.Wire()).Face(),
                                    gp_Vec(0, 20, 0)).Shape()
    # Drilled down from above the upper arm, through the next wall it meets and no further.
    hole = BRepFeat_MakeCylindricalHole()
    hole.Init(bracket, gp_Ax1(gp_Pnt(28, 0, 31), gp_Dir(0, 0, -1)))
    hole.PerformThruNext(RADIUS, False)
    if hole.Status() != BRepFeat_Status.BRepFeat_NoError:
        raise Refused(f"BRepFeat_MakeCylindricalHole could not drill the hole: {hole.Status()}")
    return hole.Shape()
