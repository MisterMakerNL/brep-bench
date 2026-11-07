"""What the OCCT implementations of the split cases share: splitting a solid by sheets, keeping
every piece, and moving pieces apart.

Splitting is BRepAlgoAPI_Splitter, the solid its argument and the sheets its tools; it returns
the pieces together, each a solid with its own faces where a sheet cut it.
"""
from OCP.BRep import BRep_Builder
from OCP.BRepAlgoAPI import BRepAlgoAPI_Splitter
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepGProp import BRepGProp
from OCP.GProp import GProp_GProps
from OCP.TopAbs import TopAbs_SOLID
from OCP.TopExp import TopExp_Explorer
from OCP.collections import List_TopoDS_Shape
from OCP.TopoDS import TopoDS, TopoDS_Compound
from OCP.gp import gp_Trsf, gp_Vec

from bench import Refused


def split(solid, sheets):
    """The pieces of `solid` split by `sheets`, each a solid."""
    arguments, tools = List_TopoDS_Shape(), List_TopoDS_Shape()
    arguments.Append(solid)
    for sheet in sheets:
        tools.Append(sheet)
    splitter = BRepAlgoAPI_Splitter()
    splitter.SetArguments(arguments)
    splitter.SetTools(tools)
    splitter.Build()
    if not splitter.IsDone():
        raise Refused("BRepAlgoAPI_Splitter could not split the solid")
    pieces = []
    explorer = TopExp_Explorer(splitter.Shape(), TopAbs_SOLID)
    while explorer.More():
        pieces.append(TopoDS.Solid(explorer.Current()))
        explorer.Next()
    return pieces


def centre(solid):
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(solid, props)
    c = props.CentreOfMass()
    return c.X(), c.Y(), c.Z()


def volume(solid):
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(solid, props)
    return props.Mass()


def moved(solid, by):
    trsf = gp_Trsf()
    trsf.SetTranslation(gp_Vec(*by))
    return BRepBuilderAPI_Transform(solid, trsf, True).Shape()


def together(solids):
    """The solids as one result, a compound of them."""
    builder, compound = BRep_Builder(), TopoDS_Compound()
    builder.MakeCompound(compound)
    for solid in solids:
        builder.Add(compound, solid)
    return compound
