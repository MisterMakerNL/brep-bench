+++
title = "A cylinder sewn from its side and two disks"
tags = ["sew", "cylinder", "planar", "seam", "analytic"]

[volume]
value = 785.3981633974482
exact = "πr²h, r = 5, h = 10"
tex = '\pi r^2 h,\quad r = 5,\ h = 10'

[area]
value = 471.23889803846896
exact = "2πrh + 2πr² = 150π"
tex = '2\pi r h + 2\pi r^2 = 150\pi'
+++

Three separate faces that share no edge or vertex:

- the side, the cylindrical face of radius 5 about the z axis from z = 0 to z = 10, made on its
  own, closed on its seam in the half plane y = 0, x ≥ 0;
- the bottom, the disk of radius 5 on the plane z = 0 centered on the z axis;
- the top, the same disk on the plane z = 10.

Sew the three into one closed shell, so that each disk's circle and the side's rim at that end
become one edge, and make the solid the shell bounds, faced outward. Do not use a cylinder
primitive or an extrusion: the case is about joining faces that were made apart.

The solid is the cylinder of radius 5 about the z axis from z = 0 to z = 10.
