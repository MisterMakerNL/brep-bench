//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two beads slid past it: two tori on one axis parallel to y through (11.5, 0, -9), the first
//! centred there with radii 6 and 2, the second centred 1.5 further along the axis with radii 6
//! and 1.5, their union slid without turning 40 along z. Everything the wheel passes through,
//! cut.
use crate::bench::cut_all;
use crate::shared::swept::stadium_prism;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let rod = make_cylinder([0.0; 3], 5.0, 20.0)?;
    // The kernel's sweep of a whole body (`shared::swept::slid`) refuses a torus (see
    // `swept/torus-slid-through-rod`), so the wheel's sweep is built as elementary geometry gives
    // it. The sweep of a union is the union of the sweeps. A torus slid along a line in its own
    // mid-plane passes over its shadow seen along the line: a point at distance ρ from the axis
    // and h from the bead's mid-plane is in the bead where (ρ − 6)² + h² ≤ r², so the slide
    // passes through it just where its distance from the axis's line, along x, is at most
    // 6 + √(r² − h²): the stadium of the bead's tube's two discs in the plane z = −9, and the
    // band between. The slide (40) is far longer than either bead is across, so within the rod
    // each bead's sweep is its stadium's prism, from the starting centre's level up 40.
    let first = stadium_prism(5.5, 17.5, 0.0, 2.0, -9.0, 40.0)?;
    let second = stadium_prism(5.5, 17.5, 1.5, 1.5, -9.0, 40.0)?;
    cut_all(rod, [first, second])
}
