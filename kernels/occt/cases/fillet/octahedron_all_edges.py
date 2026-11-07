"""An octahedron with every edge rounded, four rounds meeting at each corner. The octahedron
|x| + |y| + |z| ≤ 10 with all twelve edges rounded to radius 2, each corner closed by a patch of the
sphere of radius 2 touching its four faces: the union of every ball of radius 2 inside the
octahedron, the octahedron shrunk by 2 and grown by 2."""
from bench import other_case
from offset import offset


def build():
    # Workaround: BRepFilletAPI_MakeFillet rounds all twelve edges, and its result is valid, but
    # where four rounds meet it closes the corner with a patch of its own, not the ball's sphere:
    # its volume is 1270.7535, against 1270.7657. The solid the case asks for is the octahedron
    # shrunk by the radius and grown by it again, the union of the balls inside it, which OCCT's
    # offsets build: sharp where shrunk, and rounded by arcs where grown.
    octahedron = other_case("chamfer/octahedron-all-edges").octahedron()
    return offset(offset(octahedron, -2), 2)
