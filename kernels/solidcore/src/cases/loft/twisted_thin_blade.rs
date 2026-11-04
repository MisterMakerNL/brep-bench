//! A thin blade twisted through a whole turn: the rectangle 40 × 0.4 (−20 ≤ x ≤ 20,
//! −0.2 ≤ y ≤ 0.2) at z = 0, turned right-handed about the z axis by 6° for every unit it rises,
//! up to z = 60, its corners on four helices. That is the screw motion of pitch 60 for one turn,
//! so the blade is the rectangle's screw sweep, its walls the helicoids its sides trace (each
//! within the tolerance given, 10⁻⁴, of its helicoid: at 10⁻⁶ the sweep and its measures take
//! close to a minute), its ends flat.
use crate::bench::polygon;
use crate::Refusal;
use solidcore_ops::sweep::screw_sweep;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let section = polygon(&[[-20.0, -0.2, 0.0], [20.0, -0.2, 0.0], [20.0, 0.2, 0.0], [-20.0, 0.2, 0.0]]);
    Ok(screw_sweep(&section, [0.0; 3], [0.0, 0.0, 1.0], 60.0, 1.0, None, 1e-4)?)
}
