//! What the solidcore implementations of the boolean cases share: solidcore-ops makes its
//! cylinders about +Z only, so a cylinder along x or y is one made about z and turned.
use crate::bench::{rotated, translated};
use crate::Refusal;
use solidcore::boolean::api_brep::boolean;
use solidcore::geom::a_object::BooleanOperation;
use solidcore::kernel::{Kernel, KernelOptions, UpstreamFixes};
use solidcore_ops::{make_cylinder, Body};

/// `body` less each of `tools`, one cut after another, each the kernel's own boolean
/// (`SmApiBoolean`, `solidcore::boolean::api_brep::boolean`), on a kernel configured as the
/// runner configures ops's (every upstream fix on), the result read back into a body through its
/// record.
///
/// ops's boolean is the same kernel call, but with its result contract around it: it checks
/// both operands and the result valid and outward (a full validity check and a volume
/// integral of each) on every call. On a body that grows with every cut, those checks are
/// nearly all the time a long run of cuts takes; here the body is checked once, when it is read
/// back (and again by the runner).
pub fn cut_all_in_kernel(body: Body, tools: Vec<Body>) -> Result<Body, Refusal> {
    let mut k = Kernel::new(&KernelOptions::default());
    for name in UpstreamFixes::NAMES {
        if let Some(on) = k.fixes.flag_mut(name) {
            *on = true;
        }
    }
    let into = |k: &mut Kernel, b: Body| -> Result<solidcore::Brep, Refusal> {
        let (record, _) = b.to_record()?;
        let (brep, _) = solidcore_convert::brep::to_kernel(k, &record, true)
            .map_err(|e| format!("the record does not convert: {e:?}"))?;
        Ok(brep)
    };
    let mut brep = into(&mut k, body)?;
    for tool in tools {
        let tool = into(&mut k, tool)?;
        brep = boolean(&mut k, brep, tool, BooleanOperation::Difference)?.result;
    }
    let (record, _) = solidcore_convert::brep::to_record(&mut k, &mut brep)
        .map_err(|e| format!("the kernel's body has no record: {e:?}"))?;
    Ok(Body::from_record(&record, true)?.0)
}

/// The cylinder of `radius` and `length` along +x, its base centre at `start`: one about +Z
/// turned a quarter turn about y (z onto x), then moved.
pub fn cylinder_along_x(start: [f64; 3], radius: f64, length: f64) -> Result<Body, Refusal> {
    let c = make_cylinder([0.0; 3], radius, length)?;
    translated(rotated(c, [0.0; 3], [0.0, 1.0, 0.0], 90.0)?, start)
}

/// The cylinder of `radius` and `length` along +y, its base centre at `start`: one about +Z
/// turned a quarter turn back about x (z onto y), then moved.
pub fn cylinder_along_y(start: [f64; 3], radius: f64, length: f64) -> Result<Body, Refusal> {
    let c = make_cylinder([0.0; 3], radius, length)?;
    translated(rotated(c, [0.0; 3], [1.0, 0.0, 0.0], -90.0)?, start)
}
