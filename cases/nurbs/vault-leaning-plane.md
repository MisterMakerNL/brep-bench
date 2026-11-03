+++
title = "A weighted barrel vault cut by a plane leaning a tenth of a degree off its crown"
tags = ["nurbs", "rational", "weights", "boolean", "cut", "shallow"]

[volume]
value = 8175.287943042824
exact = "3168 + 5400·asin(4/5) less ∫₀¹² seg(x·tan α) dx, seg(δ) = 15²·acos((15 − δ)/15) − (15 − δ)√(30δ − δ²), α = 0.1°: with D = 12 tan α, ∫₀ᴰ seg = −F(15 − D), F(s) = 15²(s·acos(s/15) − √(15² − s²)) + (15² − s²)^(3/2)/3, divided by tan α"
tex = '3168 + 5400\arcsin\tfrac{4}{5} - \frac{1}{\tan\alpha}\int_0^{12\tan\alpha}\mathrm{seg}(\delta)\,d\delta,\ \alpha = 0.1^\circ'

[area]
value = 2404.909751382876
exact = "1320 + 1170·asin(4/5), less the vault's share (2·15/tan α)(√(2·15D − D²) − (15 − D)·acos((15 − D)/15)) and the arched side's segment seg(D), plus the cut's face seg(D)/(tan α cos α), D = 12 tan α"
tex = '1320 + 1170\arcsin\tfrac{4}{5} - \frac{30}{\tan\alpha}\left(\sqrt{30D-D^2} - (15-D)\arccos\tfrac{15-D}{15}\right) - \mathrm{seg}(D) + \frac{\mathrm{seg}(D)}{\tan\alpha\cos\alpha},\ D = 12\tan\alpha'
+++

The block of `nurbs/vault-tangent-plane` — the square −12 ≤ x, y ≤ 12, its floor at z = 0, its
sides upright, its top the rational B-spline surface of degree 1 along x and 2 along y whose
control points are (x, −12, 10), (x, 0, 26) and (x, 12, 10) for x = −12 and x = 12, weights 1, 3/5
and 1, which is the arch y² + (z − 1)² = 15² rising to its crown z = 16 along y = 0 — less
everything above the plane through (0, 0, 16) that holds the y direction and falls toward +x at a
tenth of a degree: z = 16 − x·tan 0.1°.

The plane passes through the crown's middle point and stands above the crown for x < 0 and below
it for x > 0, where at each x it cuts the circular segment x·tan 0.1° deep. It crosses the vault
along one curve, a parabola-like loop from the crown's middle out to the side x = 12, at a tenth of
a degree where it leaves the crown and steeper toward the side: no closed form of a plane and a
cylinder parallel to it, and a shallow crossing all along.
