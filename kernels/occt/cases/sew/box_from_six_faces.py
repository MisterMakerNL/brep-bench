"""A box sewn from its six faces. Six separate rectangles, the faces of the box from the origin to
(20, 10, 5), each made on its own, sewn into one closed shell and made the solid it bounds."""
from sew import box_faces, sewn


def build():
    return sewn(box_faces((0, 0, 0), (20, 10, 5)))
