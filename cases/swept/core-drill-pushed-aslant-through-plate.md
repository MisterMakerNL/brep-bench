+++
title = "A core drill pushed along its leaning axis through a plate"
tags = ["swept", "solid-sweep", "slide", "cylinder", "tube", "oblique", "two-lumps", "boolean", "cut"]
origin = "geomsolver #721, box G7 (an annular shadow under a slide along the tool's axis)"

[volume]
value = 1541.958420344505
exact = "1600 − 4·π(2.5² − 1.5²)/cos 30°: each level loses the annulus between two ellipses"
tex = '1600 - \tfrac{4\pi(2.5^2 - 1.5^2)}{\cos 30^\circ}'

[area]
value = 1207.0623694832425
exact = "1120 − 2·π(2.5² − 1.5²)/cos 30° + the walls 2π(2.5 + 1.5)·4/cos 30°"
tex = '1120 + \tfrac{24\pi}{\cos 30^\circ}'
+++

A plate, the box from (−10, −10, 0) to (10, 10, 4), with an annular cut made right through it by a
core drill pushed along its own leaning axis.

The drill is a tube: everything between 1.5 and 2.5 from its axis, which is the line through (0, 0,
2) along the direction u = (sin 30°, 0, cos 30°), leaning 30° from upright toward +x. As placed, the
tube runs along its axis from 18 to 6 before (0, 0, 2), its ends square to its axis, wholly below
the plate.

It slides along u by 24, to run from 6 to 18 past (0, 0, 2), wholly above the plate. Cut from the
plate everything the tube passes through.

The tube's shadow along the push is an annulus, so it sweeps two oblique cylinders through the
plate. The core inside the inner one is cut free of the plate and stays: the result is two separate
solids, the plate with its oblique hole and the core, an oblique cylinder of radius 1.5 cut by the
planes z = 0 and z = 4.
