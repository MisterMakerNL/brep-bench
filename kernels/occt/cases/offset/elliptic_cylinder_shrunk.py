"""An elliptic cylinder shrunk by 1. The ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0 extruded up
to z = 10 (geometry/elliptic-cylinder), shrunk by 1: every point at least 1 from its outside."""
from bench import other_case
from offset import offset


def build():
    # OCCT builds this solid (its section at z = 5 bounds the offset ellipse's area), but the
    # runner's measures of it are off: BRepGProp's adaptive integration over the side's offset
    # surface strays further the finer it is asked to be, by 2e-2 at 1e-10 and 5e-2 at 1e-6.
    return offset(other_case("geometry/elliptic-cylinder").build(), -1)
