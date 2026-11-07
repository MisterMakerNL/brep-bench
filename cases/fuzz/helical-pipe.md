+++
title = "A helical pipe, twenty ways"
tags = ["fuzz", "sweep", "helix"]

[params]
R = [4, 20]
r = ["0.05*R", "0.4*R"]
p = ["3*r", "4*R"]
n = [0.5, 4]
hand = { choice = ["right", "left"] }

[let]
L = "n*sqrt((2*pi*R)**2 + p**2)"

[volume]
formula = "pi*r**2*L"
exact = "πr²L, L = n√((2πR)² + p²)"
tex = '\pi r^2 L,\quad L = n\sqrt{(2\pi R)^2 + p^2}'

[area]
formula = "2*pi*r*L + 2*pi*r**2"
exact = "2πrL + 2πr²"
tex = '2\pi r L + 2\pi r^2'
+++

A pipe of circular section, radius `r`, following a helix about the z axis. The helix has radius
`R` and rises `p` per turn. It starts at (`R`, 0, 0) and turns right-handed (from +x toward +y,
rising) when `hand` is `right`, left-handed (from +x toward −y, rising) when it is `left`, through
`n` turns, which need not be whole. The section stays square to the path, and the pipe's ends are
flat.

`sweep/helical-pipe`, of any proportions, either hand, through any part of a turn.
