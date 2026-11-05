// A thin blade: the rectangle 40 x 0.4 centred on the z axis, from z = 0 to 60, turned about the
// z axis right-handed by 6° for every unit of height, a whole turn in all; its corners run along
// four helices. popkern's loft takes no guide curves: the rectangle is swept up the z axis,
// twisted a whole turn on the way, which turns it steadily with height. The path is the edge
// along the z axis of a block that only carries it.
let m = model();
let rail = m.profile(xy()).rect(1.0, 1.0).at(0.5, 0.5).sketch("rail_section").extrude(blind(60.0));
let blade = m.profile(xy()).rect(40.0, 0.4).sketch("blade_section");
blade.sweep(rail, edges().direction("z").at_vertex([0.0, 0.0, 0.0]), #{ twist: 360.0, label: "blade" });
m
