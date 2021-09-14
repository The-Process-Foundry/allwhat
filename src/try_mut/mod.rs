//! Mutate value with rollback on error
//!
//! TODO: Convert this to use Patchwork as tracking the mutation, Historic (possibly) to be the
//! stash
//!
//! This is totally experimental. I have no idea if this is reasonable enough to be helpful or just
//! shuffling boiler-plate. There is a lot of boilerplate that needs to be implemented that is best
//! left to a derive

mod traits;
pub use traits::*;

pub mod impls;
pub use impls::*;
