//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from -1 to 1
//! and polar angle 0° to 60°, grooved by a cutter turned past it: the torus centred on (5, 0, 0)
//! with radii 2.5 and 2, its axis leaning from z towards +y by 25°, turned rigidly about the z
//! axis from -140° to +140°. Everything the cutter passes through, cut from the block.
//!
//! The groove is the kernel's own sweep of the whole torus about the z axis
//! (`SmApiNonManifoldRotationalSweep`, `shared::swept::swung`). The leaning torus's wall folds
//! over itself as it turns, so its sweep is bounded by an envelope that is none of its own
//! curves.
use crate::bench::{arc, cut, line, rotated, translated};
use crate::shared::swept::swung;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body, Refusal> {
    let section = [
        line([8.0, 0.0, -1.0], [12.0, 0.0, -1.0]),
        line([12.0, 0.0, -1.0], [12.0, 0.0, 1.0]),
        line([12.0, 0.0, 1.0], [8.0, 0.0, 1.0]),
        line([8.0, 0.0, 1.0], [8.0, 0.0, -1.0]),
    ];
    let block = revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], PI / 3.0, true)?;
    // the torus about the z axis through the origin: its tube's circle revolved a whole turn
    let tube = [arc([2.5, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 2.0, 0.0, TAU)];
    let torus = revolve(&tube, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?;
    // its axis leaned from z towards +y (a turn of -25° about x), moved to (5, 0, 0), and set at
    // the start of its turn, -140° about z
    let torus = rotated(torus, [0.0; 3], [1.0, 0.0, 0.0], -25.0)?;
    let torus = translated(torus, [5.0, 0.0, 0.0])?;
    let torus = rotated(torus, [0.0; 3], [0.0, 0.0, 1.0], -140.0)?;
    let groove = swung(&torus, [0.0; 3], [0.0, 0.0, 1.0], 280.0)?;
    cut(block, groove)
}
