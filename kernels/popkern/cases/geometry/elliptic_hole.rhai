// A block −12..12 × −8..8 × 0..8 less an elliptic cylinder x²/36 + y²/9 < 1 run past both faces.
let m = model();
let block = m.profile(xy()).rect(24.0, 16.0).sketch("block").extrude(blind(8.0));
let bore = m.tool(|m| m.profile(xy_at(-1.0)).ellipse(6.0, 3.0).sketch("bore").extrude(blind(10.0)));
block = block.subtract(bore, #{ label: "hole" });
m
