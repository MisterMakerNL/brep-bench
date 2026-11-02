"""A post extruded up to the inside of a hollow dome. A closed hollow dome with a post inside it.

The dome: a plate, the disc of radius 20 about the z axis from z = −2 to z = 0, and on it a
hemispherical shell, everything above z = 0 between 18 and 20 from the origin. Inside them is a
closed cavity, the half ball of radius 18 about the origin above the plate.

The post: the disc of radius 3 about the z axis on the cavity's floor (the plate's top, z = 0),
extruded up (along +z) until it meets the inside of the dome, the sphere of radius 18, and joined
to the dome. So the post's side is a cylinder of radius 3 from z = 0 to z = √315 ≈ 17.748, and
its top is the cap of the sphere above the disc, rising to z = 18 on the axis. The cavity is
left as a ring about the post."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

OUTER, INNER, PLATE = 20, 18, 2
POST = 3


def face(shape, kind, test):
    explorer = TopExp_Explorer(shape, TopAbs_FACE)
    while explorer.More():
        f = TopoDS.Face(explorer.Current())
        surface = BRepAdaptor_Surface(f)
        if surface.GetType() == kind and test(surface):
            return f
        explorer.Next()
    raise Refused(f"the dome has no such face ({kind})")


def build():
    plate = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -PLATE), gp_Dir(0, 0, 1)),
                                     OUTER, PLATE).Shape()
    shell = BRepAlgoAPI_Cut(BRepPrimAPI_MakeSphere(OUTER, 0, math.pi / 2).Shape(),
                            BRepPrimAPI_MakeSphere(INNER, 0, math.pi / 2).Shape())
    dome = BRepAlgoAPI_Fuse(plate, shell.Shape())
    if not dome.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the shell to the plate")
    dome = dome.Shape()
    # The cavity's floor, z = 0 (the plate's top, now inside the dome), and the dome's inside.
    floor = face(dome, GeomAbs_SurfaceType.GeomAbs_Plane,
                 lambda s: abs(s.Plane().Location().Z()) < 1e-9)
    inside = face(dome, GeomAbs_SurfaceType.GeomAbs_Sphere,
                  lambda s: abs(s.Sphere().Radius() - INNER) < 1e-9)
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), POST))
    disc = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(circle.Edge()).Wire()).Face()
    post = BRepFeat_MakePrism(dome, disc, floor, gp_Dir(0, 0, 1), 1, True)
    post.Perform(inside)
    if not post.IsDone():
        raise Refused("BRepFeat_MakePrism could not extrude the post: "
                      f"{post.CurrentStatusError()}")
    return post.Shape()
