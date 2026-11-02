"""A square pad extruded down to a cylinder. A cylinder of radius 10 lying along the x axis, from x
= −20 to x = 20, with a square pad on top. The pad is the square |x| ≤ 3, |y| ≤ 3 in the plane z =
15, extruded down (along −z) until it meets the cylinder's side, and joined to the cylinder.

So the pad's top is that square, its four sides are vertical, and its bottom is the part of the
cylinder's side under the square: it reaches z = 10 at its middle, y = 0, and z = √91 ≈ 9.539 at
its sides y = ±3. The pad and the cylinder are one solid."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS, TopoDS_Face
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS, HALF_LENGTH = 10, 20
HALF_SIDE, SKETCH_Z = 3, 15


def side(shape):
    """The cylinder's side, the face the pad runs down to."""
    explorer = TopExp_Explorer(shape, TopAbs_FACE)
    while explorer.More():
        face = TopoDS.Face(explorer.Current())
        if BRepAdaptor_Surface(face).GetType() == GeomAbs_SurfaceType.GeomAbs_Cylinder:
            return face
        explorer.Next()
    raise Refused("the cylinder has no cylindrical face")


def build():
    cylinder = BRepPrimAPI_MakeCylinder(
        gp_Ax2(gp_Pnt(-HALF_LENGTH, 0, 0), gp_Dir(1, 0, 0)), RADIUS, 2 * HALF_LENGTH).Shape()
    square = BRepBuilderAPI_MakePolygon()
    for x, y in [(-1, -1), (1, -1), (1, 1), (-1, 1)]:
        square.Add(gp_Pnt(x * HALF_SIDE, y * HALF_SIDE, SKETCH_Z))
    square.Close()
    sketch = BRepBuilderAPI_MakeFace(square.Wire()).Face()
    # A boss (fuse) from the sketch, which lies on no face of the cylinder, down to its side.
    pad = BRepFeat_MakePrism(cylinder, sketch, TopoDS_Face(), gp_Dir(0, 0, -1), 1, True)
    pad.Perform(side(cylinder))
    if not pad.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not extrude the pad: {pad.CurrentStatusError()}")
    return pad.Shape()
