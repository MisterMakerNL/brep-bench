"""A pocket with rounded corners and its walls drafted. The block from the origin to (40, 30, 15),
with a pocket cut into its top: the box from (8, 8, 5) to (32, 22, 15), its four vertical edges
rounded to radius 4, open at the block's top face, its floor at z = 5. The pocket's walls are four
planes and four quarter cylinders of radius 4 about the vertical lines through (12, 12), (28, 12),
(28, 18) and (12, 18), each tangent to the planes beside it: concave rounds, seen from the block's
material.

Draft all eight walls by 5°, with the pull direction +z and the neutral plane z = 5, the pocket's
floor, so that each leans 5° from the z direction, away from the pocket's middle as it rises. Each
plane turns about its bottom edge; each quarter cylinder becomes a quarter cone about the same axis,
its radius 4 + (z − 5)·tan 5° at height z, so that it stays tangent to the planes beside it. The
pocket's floor and the block's faces are not drafted; the opening in the block's top grows to meet
the drafted walls.

The result is the block less the pocket, whose section at height z is its floor's outline, the
rounded rectangle, moved (z − 5)·tan 5° outward: its straight sides outset by that and its corners'
radii 4 + (z − 5)·tan 5° about the same centres."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet2d
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.TopAbs import TopAbs_VERTEX
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt, gp_Vec

from bench import Refused
from draft import drafted
from shell import face_through

BLOCK = (40, 30, 15)
POCKET = ((8, 8, 5), (32, 22, 15))
RADIUS = 4
DRAFT = 5  # degrees: the walls lean away from the pocket's middle as they rise


def rounded_rectangle(x0, y0, x1, y1, z):
    """The rectangle from (x0, y0) to (x1, y1) at height z, its corners rounded to RADIUS."""
    face = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), x0, x1, y0, y1).Face()
    rounds = BRepFilletAPI_MakeFillet2d(face)
    corners = TopExp_Explorer(face, TopAbs_VERTEX)
    seen = []
    while corners.More():
        vertex = TopoDS.Vertex(corners.Current())
        if not any(vertex.IsSame(v) for v in seen):
            seen.append(vertex)
            rounds.AddFillet(vertex, RADIUS)
        corners.Next()
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet2d could not round the pocket's corners")
    return rounds.Shape()


def build():
    block = BRepPrimAPI_MakeBox(*BLOCK).Shape()
    (x0, y0, z0), (x1, y1, z1) = POCKET
    # the pocket's prism runs past the top, so that the cut leaves no sliver of the top face
    floor = rounded_rectangle(x0, y0, x1, y1, z0)
    pocket = BRepPrimAPI_MakePrism(floor, gp_Vec(0, 0, z1 - z0 + 1)).Shape()
    cut = BRepAlgoAPI_Cut(block, pocket)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    part = cut.Shape()
    xm, ym, zm = (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2
    k = RADIUS * (1 - 2 ** -0.5)
    points = [(x0, ym, zm), (x1, ym, zm), (xm, y0, zm), (xm, y1, zm),
              (x0 + k, y0 + k, zm), (x1 - k, y0 + k, zm),
              (x1 - k, y1 - k, zm), (x0 + k, y1 - k, zm)]
    return drafted(part, [face_through(part, p) for p in points], DRAFT, neutral=(0, 0, z0))
