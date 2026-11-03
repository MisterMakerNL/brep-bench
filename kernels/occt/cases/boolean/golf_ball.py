"""A golf ball. A ball of radius 10 centred on the origin, with 62 dimples, each cut by a ball of
radius 1.2 centred 10.6 out towards one of the 12 vertices, 30 edge midpoints and 20 face centres
of the icosahedron with vertices (0, ±1, ±φ), (±1, ±φ, 0) and (±φ, 0, ±1). Each ball's poles on
the vertical through its centre; each dimple its own cut."""
from itertools import combinations
from math import dist, sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS = 10
DIMPLE_RADIUS = 1.2
DIMPLE_CENTRE = 10.6


def unit(p):
    n = sqrt(sum(x * x for x in p))
    return tuple(x / n for x in p)


def directions():
    """The icosahedron's vertices, edges' midpoints and faces' centres, as unit vectors."""
    phi = (1 + sqrt(5)) / 2
    vertices = []
    for a in (-1, 1):
        for b in (-phi, phi):
            vertices += [(0, a, b), (a, b, 0), (b, 0, a)]
    # neighbours are an edge, 2, apart
    def edge(i, j):
        return abs(dist(vertices[i], vertices[j]) - 2) < 1e-9
    out = [unit(v) for v in vertices]
    out += [unit([x + y for x, y in zip(vertices[i], vertices[j])])
            for i, j in combinations(range(12), 2) if edge(i, j)]
    out += [unit([x + y + z for x, y, z in zip(vertices[i], vertices[j], vertices[k])])
            for i, j, k in combinations(range(12), 3) if edge(i, j) and edge(j, k) and edge(i, k)]
    return out


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    dimples = directions()
    assert len(dimples) == 62
    for d in dimples:
        cutter = BRepPrimAPI_MakeSphere(gp_Pnt(*(DIMPLE_CENTRE * x for x in d)), DIMPLE_RADIUS).Shape()
        cut = BRepAlgoAPI_Cut(ball, cutter)
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a dimple")
        ball = cut.Shape()
    return ball
