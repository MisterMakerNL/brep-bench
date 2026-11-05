//! The brep-bench runner for cadrs's kernel (https://github.com/rvdende/cadrs, the crate
//! `cadrs_kernel`).
//!
//!     brep-bench-cadrs CASE OUT.stl
//!     brep-bench-cadrs --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, in a session of cadrs's native kernel (`DefaultKernel`)
//! through the `Kernel` trait, the API cadrs's Part Studio features call. It then measures the
//! body with the kernel's own tools: its volume and area integrated over the exact B-rep
//! (`Kernel::mass_properties`), its faces by surface kind, and the kernel's full validity check
//! (`Body::check_with(CheckLevel::Full)`). It meshes the body with the kernel's tessellator
//! (`Kernel::tessellate`) into OUT.stl and prints one line of JSON. An operation's error is a
//! refusal: the kernel declined, saying why. A case may also refuse in its own words, where the
//! kernel has no operation for what it describes. The runner never reads the case's description.
use cadrs_kernel::native::{CheckLevel, Surface};
use cadrs_kernel::{BodyClass, BodyId, DefaultKernel, Kernel, KernelError, Tessellation};
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod bench;
#[allow(dead_code, unused_imports)]
mod shared;

/// A case's implementation: the body it builds in the session, or why it was not built.
type Build = fn(&mut DefaultKernel) -> Result<BodyId, Refusal>;

/// Why a case was not built: the kernel's error, in its own words, or a case that refuses in its
/// own because the kernel has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<KernelError> for Refusal {
    fn from(e: KernelError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<&str> for Refusal {
    fn from(why: &str) -> Self {
        Refusal(why.to_owned())
    }
}

impl From<String> for Refusal {
    fn from(why: String) -> Self {
        Refusal(why)
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One module per case file, and `CASES`, written by `build.rs`.
mod cases {
    use super::{Build, Refusal};
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs(_) => "bspline",
        _ => "other",
    }
}

/// A JSON string.
fn quoted(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn refused(why: &str) -> String {
    format!("{{\"status\":\"refused\",\"reason\":{}}}", quoted(why))
}

/// The triangles written as binary STL. The tessellator samples each edge once, so the faces
/// either side of it share its points exactly.
fn write_stl(path: &str, mesh: &cadrs_kernel::TriMesh) -> std::io::Result<()> {
    let mut out = Vec::with_capacity(84 + 50 * mesh.indices.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(mesh.indices.len() as u32).to_le_bytes());
    for t in &mesh.indices {
        let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
        let n = (b - a).cross(&(c - a));
        let n = if n.norm() > 0.0 { n.normalize() } else { n };
        for x in [n.x, n.y, n.z, a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z] {
            out.extend_from_slice(&(x as f32).to_le_bytes());
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out)
}

/// The reply for a body built: its measures, once its triangles are written. The mesh keeps
/// within a ten-thousandth of the body's size, or 1/200 of its thickness (its volume over its
/// area) where that is less: the reference runner's rule (`kernels/occt/run.py`).
fn measured(k: &DefaultKernel, body: BodyId, out: &str) -> Result<String, Refusal> {
    match k.body_class(body)? {
        BodyClass::Solid => {}
        other => return Err(format!("the result is not a solid but {other:?}").into()),
    }
    let m = k.mass_properties(body)?;
    let (volume, area) = (m.volume, m.surface_area);
    let native = k.get(body)?;
    let valid = native.check_with(CheckLevel::Full).is_empty();
    let mut faces = BTreeMap::new();
    for f in native.faces() {
        *faces.entry(kind(&f.surf)).or_insert(0) += 1;
    }
    let faces = faces.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect::<Vec<_>>().join(",");

    let size = k.bounding_box(body)?.diagonal();
    let thickness = if area > 0.0 && volume > 0.0 { volume / area } else { size };
    let deflection = (1e-4 * size).min(5e-3 * thickness).max(2e-7);
    let quality = Tessellation { deflection, angle: 5f64.to_radians() };
    let mesh = k.tessellate(body, quality).map_err(|e| format!("not meshed: {e}"))?;
    write_stl(out, &mesh).map_err(|e| format!("{out} not written: {e}"))?;
    Ok(format!(
        "{{\"status\":\"ok\",\"volume\":{volume:e},\"area\":{area:e},\"faces\":{{{faces}}},\"valid\":{valid}}}"
    ))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!(
            "cadrs_kernel {} (brep-bench runner {})",
            env!("KERNEL_VERSION"),
            env!("CARGO_PKG_VERSION")
        );
        return ExitCode::SUCCESS;
    }
    let [_, case, out] = &args[..] else {
        eprintln!("usage: brep-bench-cadrs CASE OUT.stl");
        return ExitCode::from(2);
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    let mut k = DefaultKernel::new();
    let reply = match build(&mut k).and_then(|body| measured(&k, body, out)) {
        Ok(reply) => reply,
        Err(why) => refused(&why.to_string()),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
