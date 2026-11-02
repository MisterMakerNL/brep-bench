"""An L-block with its sides drafted, about a concave edge. An L-shaped block: the outline through
(0, 0), (30, 0), (30, 10), (10, 10), (10, 25) and (0, 25) in the plane z = 0, extruded from z = 0 to
z = 12. Its six sides meet along five convex vertical edges and one concave one, through (10, 10).

Draft all six sides by 6°, with the pull direction +z and the neutral plane z = 0, the plane of its
floor: each side turns about its bottom edge until it leans 6° from the z direction, inward as it
rises, away from the side that the block's material is not on. The floor and the top are not
drafted; the top is trimmed to the drafted sides, and the sides meet one another along new edges
where their planes cross, the two at the concave edge too.

The result has eight faces. Its section at height z is the L moved d = z·tan 6° into its material,
each side parallel to itself: the outline through (d, d), (30 − d, d), (30 − d, 10 − d), (10 − d, 10
− d), (10 − d, 25 − d) and (d, 25 − d)."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from draft import drafted
from shell import face_through

OUTLINE = [(0, 0), (30, 0), (30, 10), (10, 10), (10, 25), (0, 25)]
HEIGHT = 12
DRAFT = 6  # degrees, inward as the sides rise


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in OUTLINE:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    floor = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    block = BRepPrimAPI_MakePrism(floor, gp_Vec(0, 0, HEIGHT)).Shape()
    # each side, by the middle of its edge in the outline, halfway up
    n = len(OUTLINE)
    middles = [((x + OUTLINE[(k + 1) % n][0]) / 2, (y + OUTLINE[(k + 1) % n][1]) / 2, HEIGHT / 2)
               for k, (x, y) in enumerate(OUTLINE)]
    return drafted(block, [face_through(block, p) for p in middles], DRAFT)
