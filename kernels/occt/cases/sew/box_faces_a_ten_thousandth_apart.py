"""A box healed from faces a ten-thousandth apart. The six faces of the box from the origin to
(20, 10, 5), each shrunk by 0.0001 on every side so that no two meet, sewn into one closed shell
with a tolerance of 0.001 and healed, each face extended along its plane to meet its neighbours
exactly."""
from bench import Refused


def build():
    # BRepBuilderAPI_Sewing joins the faces within the tolerance, but only by widening the
    # tolerance of the edges it shares between them: each face keeps the boundary it was given,
    # so the solid is still bounded by the shrunk rectangles (its area 699.972, not 700).
    # ShapeFix_Shape and ShapeUpgrade_UnifySameDomain leave those boundaries as they are too.
    raise Refused("OCCT sews faces within a tolerance but cannot heal them: nothing extends a "
                  "sewn face along its surface to meet its neighbours exactly")
