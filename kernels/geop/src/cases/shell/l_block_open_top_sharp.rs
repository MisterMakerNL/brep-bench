//! An L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded
//! 8 up, shelled inward to a wall 1 thick, open at its top, every pair of moved faces extended
//! until they meet, the concave edge included. The plate is extruded from its outline and shelled
//! by geop's shell, which meets moved faces sharp; the top is picked by a point on it.
use crate::shared::hollow::{prism, shell};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let outline = [[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]];
    prism(b, "plate", &outline, 8.0)?;
    shell(b, "cup", &[[2.0, 2.0, 8.0]], 1.0, false)
}
