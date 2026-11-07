"""What the OCCT implementations of the vertex-blend cases share: picking edges by where they run,
and rounding them with BRepFilletAPI_MakeFillet."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused


def edges_where(solid, test):
    """The edges of `solid` whose middle point satisfies `test`, a test of a gp_Pnt, each once."""
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        middle = curve.Value((curve.FirstParameter() + curve.LastParameter()) / 2)
        if test(middle) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
        edges.Next()
    return found


def rounded(solid, radius, edges, what):
    """`solid` with `edges` rounded to `radius`; Refused, naming `what`, if OCCT declines."""
    fillet = BRepFilletAPI_MakeFillet(solid)
    for edge in edges:
        fillet.Add(radius, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused(f"BRepFilletAPI_MakeFillet could not round {what}")
    return fillet.Shape()
