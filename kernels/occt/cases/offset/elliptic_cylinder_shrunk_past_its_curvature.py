"""An elliptic cylinder shrunk past its curvature. The ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0
extruded up to z = 10 (geometry/elliptic-cylinder), shrunk by 2.5: every point at least 2.5 from its
outside, more than the ellipse's least radius of curvature, 1.8."""
from bench import other_case
from offset import offset


def build():
    return offset(other_case("geometry/elliptic-cylinder").build(), -2.5)
