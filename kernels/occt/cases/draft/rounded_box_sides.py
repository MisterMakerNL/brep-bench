"""A rounded box with its sides drafted, the rounds with them. The box from the origin to (30, 20,
10), with its four vertical edges rounded to radius 4: its sides are four planes and four quarter
cylinders of radius 4, about the vertical lines through (4, 4), (26, 4), (26, 16) and (4, 16), each
tangent to the planes beside it.

Draft all eight sides by 5°, with the pull direction +z and the neutral plane z = 0, the plane of
its floor, so that each leans 5° from the z direction, inward as it rises. Each plane turns about
its bottom edge. Each quarter cylinder becomes a quarter cone about the same axis, through the same
quarter circle at z = 0, its radius 4 − z·tan 5° at height z, so that it stays tangent to the
drafted planes beside it. The floor and the top are not drafted; the top is trimmed to the drafted
sides.

The result has ten faces. Its section at height z is the floor's outline, the rounded rectangle,
moved z·tan 5° inward: its straight sides inset z·tan 5° and its corners' radii 4 − z·tan 5° about
the same centres. At the top they are inset 10·tan 5° ≈ 0.875, the corners of radius about 3.125."""
from draft import box_sides, drafted, rounded_box

SIZE = (30, 20, 10)
RADIUS = 4
DRAFT = 5  # degrees, inward as the sides rise


def build():
    box = rounded_box((0, 0, 0), SIZE, RADIUS)
    # the four planes and the four rounds, the rounds becoming cones
    return drafted(box, box_sides(box, (0, 0, 0), SIZE, RADIUS), DRAFT)
