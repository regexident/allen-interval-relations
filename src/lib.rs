mod bound;
mod discreteness;

pub use self::{bound::*, discreteness::*};

mod sealed {
    pub trait Sealed {}
}
