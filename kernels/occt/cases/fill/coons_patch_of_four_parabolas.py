"""A block capped by the Coons patch of four parabolas. Quadratic Bézier arcs on y = 0, x = 10,
y = 10 and x = 0, from corner to corner of the square from (0, 0) to (10, 10) at z = 10, through the
control points (5, 0, 14), (10, 5, 6), (5, 10, 6) and (0, 5, 14), filled by their bilinearly
blended Coons patch, and the solid under it down to z = 0."""
from fill import capped


def build():
    return capped([[(0, 0, 10), (5, 0, 14), (10, 0, 10)],
                   [(10, 0, 10), (10, 5, 6), (10, 10, 10)],
                   [(10, 10, 10), (5, 10, 6), (0, 10, 10)],
                   [(0, 10, 10), (0, 5, 14), (0, 0, 10)]])
