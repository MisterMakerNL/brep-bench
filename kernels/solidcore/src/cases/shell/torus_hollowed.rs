//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 6,
//! hollowed to a wall 0.5 thick with no opening: the torus less the torus of tube radius 1.5. A
//! shell with no face opened.
use crate::bench::order;
use crate::Refusal;
use solidcore_ops::{make_torus, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let ring = make_torus([0.0, 0.0, 0.0], 6.0, 2.0)?;
    shell(&ring, &order(&ring)?, &[], -0.5)?;
    Ok(ring)
}
