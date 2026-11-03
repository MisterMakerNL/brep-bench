//! The vault block of `nurbs/vault-tangent-plane`, less everything above the plane through its
//! crown's middle that falls toward +x at a tenth of a degree.
use cad_kernel::build::kept;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let block = super::vault_tangent_plane::block()?;
    let a = 0.1f64.to_radians();
    // kept: the side below the plane, against its upward normal (sin α, 0, cos α)
    kept(&block, [0., 0., 16.], [-a.sin(), 0., -a.cos()], 0.)
}
