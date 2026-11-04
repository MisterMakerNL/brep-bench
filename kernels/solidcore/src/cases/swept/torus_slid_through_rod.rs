//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel slid
//! past it: the torus with its axis parallel to the y axis, radii 6 and 2, slid without turning
//! along z from centre (11.5, 0, -9), wholly below the rod, to (11.5, 0, 29), wholly above it.
//! Everything the wheel passes through, cut.
use crate::bench::cut;
use crate::shared::swept::stadium_prism;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let rod = make_cylinder([0.0; 3], 5.0, 20.0)?;
    // The kernel's sweep of a whole body (`shared::swept::slid`) refuses the torus
    // (`sweep::trans_sweep_geometry: upstream status 1001`: its faces fold over themselves where
    // the slide runs along them), so the wheel's sweep is built as elementary geometry gives it.
    // A point at distance ρ from the wheel's axis and y from its mid-plane is in the wheel where
    // (ρ − 6)² + y² ≤ 2²; as the wheel slides along z, a point level with its centre and
    // |x − 11.5| from it sees every ρ from |x − 11.5| up, so it is passed through just where
    // |x − 11.5| ≤ 6 + √(2² − y²): the stadium of the tube's two discs in the plane z = −9,
    // about (5.5, 0) and (17.5, 0), and the band between. A point above that level, up to the
    // last centre's, is passed through as well, the slide (38) being far longer than the wheel
    // is across (16); beyond those levels the sweep is the wheel at each end, wholly below and
    // above the rod. So within the rod the sweep is the stadium's prism from z = −9 to 29.
    let swept = stadium_prism(5.5, 17.5, 0.0, 2.0, -9.0, 38.0)?;
    cut(rod, swept)
}
