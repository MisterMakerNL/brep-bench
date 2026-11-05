// The rectangle (0, 0, 0)-(10, 8, 0) thickened by 2 to its +z side: a sheet facing +z grown
// outward along its normal.
let m = model();
let s = m.sheet_rect(10.0, 8.0, #{ label: "sheet" });
s = s.thicken(2.0, #{ side: "outward", label: "thick" });
m
