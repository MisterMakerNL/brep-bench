"""A NURBS ball grown. The ball of radius 5 about the origin bounded by one rational B-spline face,
the standard biquadratic sphere of geometry/nurbs-sphere, grown by 2: its face offset outward along
its normal, through its poles, which leaves the ball of radius 7."""
from bench import other_case
from offset import offset


def build():
    # The offset by join, as the other offset cases make theirs, declines this face. OCCT's
    # simple offset (PerformBySimple) does build a solid, but approximates the offset surface:
    # its volume is 1442.67, against 1436.76.
    return offset(other_case("geometry/nurbs-sphere").build(), 2)
