"""A box drafted about a plane through its middle. A box from (−15, −10, 0) to (15, 10, 16), its
four sides drafted 4° about the plane through its middle, z = 8: the solid is pulled along +z, and
each side turns 4° about its line on z = 8 so that it leans in as it rises. The base and the top
stay horizontal.

So the section at height z is the 30 × 20 rectangle moved (z − 8)·tan 4° in on every side: it
is 30 × 20 at z = 8, larger below (the base is (30 + 16·tan 4°) × (20 + 16·tan 4°), about
31.119 × 21.119) and smaller above (the top about 28.881 × 18.881)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from draft import drafted


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(-15, -10, 0), gp_Pnt(15, 10, 16)).Shape()
    return drafted(box, 4, 8)
