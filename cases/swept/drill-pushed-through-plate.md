+++
title = "A hole bored by a drill pushed along its own axis"
tags = ["swept", "solid-sweep", "slide", "revolution", "cone", "ring-contact", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/drill.sv"

[volume]
value = 749.73451754256331
exact = "20·10·4 − π2²·4"
tex = '800 - 16\pi'

[area]
value = 665.13274122871835
exact = "640 − 2·π2² + 2π·2·4"
tex = '640 + 8\pi'
+++

A plate, the box from (0, −5, 0) to (20, 5, 4), with a hole bored through it by a drill.

The drill is a solid of revolution about the vertical line through (10, 0): a cylinder of radius 2
from z = 6 + 2/tan 59° up to z = 14, ended below in a cone of 118° (each side 59° from the axis)
whose point is (10, 0, 6).

It slides without turning straight down along its axis by 20, from where its point is 2 above the
plate to where its top is 6 below it. Cut from the plate everything the drill passes through.

Every circle of the drill about its axis meets the motion at one angle all round, so it touches all
round or nowhere, and the circle of greatest radius sweeps the hole's wall. The drill passes wholly
through, so the hole is the cylinder of radius 2, whatever its point.
