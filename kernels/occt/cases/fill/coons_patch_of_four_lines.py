"""A block capped by the Coons patch of four straight edges. The edges from (0, 0, 5) to
(10, 0, 5), (10, 10, 15), (0, 10, 5) and back, filled by their bilinearly blended Coons patch, and
the solid under it down to z = 0 over the square from (0, 0) to (10, 10)."""
from fill import capped


def build():
    corners = [(0, 0, 5), (10, 0, 5), (10, 10, 15), (0, 10, 5)]
    return capped([[corners[i], corners[(i + 1) % 4]] for i in range(4)])
