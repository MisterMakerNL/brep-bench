// The regular tetrahedron with corners (3, 3, 3), (3, -3, -3), (-3, 3, -3), (-3, -3, 3): popkern has
// no polyhedron primitive, so it is the common part of four slabs, each sketched on one face's
// plane (through the face's centre) and extruded inward. Grown by 1.
let m = model();
let faces_of = [
    [[-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]],
    [[-1.0, 1.0, 1.0], [1.0, -1.0, -1.0]],
    [[1.0, -1.0, 1.0], [-1.0, 1.0, -1.0]],
    [[1.0, 1.0, -1.0], [-1.0, -1.0, 1.0]],
];
let t = m.profile(plane_at(faces_of[0][0], faces_of[0][1])).rect(40.0, 40.0)
    .sketch("slab_0").extrude(blind(20.0));
for i in 1..4 {
    let f = faces_of[i];
    let slab = m.tool(|m| m.profile(plane_at(f[0], f[1])).rect(40.0, 40.0)
        .sketch(`slab_${i}`).extrude(blind(20.0)));
    t = t.intersect(slab, #{ label: `face_${i}` });
}
t = t.offset_solid(1.0, #{ label: "grown" });
m
