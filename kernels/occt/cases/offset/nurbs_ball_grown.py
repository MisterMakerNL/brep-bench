"""A NURBS ball grown. The ball of radius 5 about the origin bounded by one rational B-spline face,
the standard biquadratic sphere of geometry/nurbs-sphere, grown by 2: its face offset outward along
its normal, through its poles, which leaves the ball of radius 7."""
import importlib.util
import os

from offset import offset

HERE = os.path.dirname(os.path.abspath(__file__))


def nurbs_ball():
    """The ball of geometry/nurbs-sphere, built as that case builds it."""
    path = os.path.join(HERE, "..", "geometry", "nurbs_sphere.py")
    spec = importlib.util.spec_from_file_location("cases.geometry.nurbs_sphere", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.build()


def build():
    # The offset by join, as the other offset cases make theirs, declines this face. OCCT's
    # simple offset (PerformBySimple) does build a solid, but approximates the offset surface:
    # its volume is 1442.67, against 1436.76.
    return offset(nurbs_ball(), 2)
