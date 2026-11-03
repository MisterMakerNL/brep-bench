//! The vault block of `nurbs/vault-tangent-plane`, less everything above the plane a thousandth
//! below its crown.
use cad_kernel::build::kept;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let block = super::vault_tangent_plane::block()?;
    kept(&block, [0., 0., 16. - 1e-3], [0., 0., -1.], 0.)
}
