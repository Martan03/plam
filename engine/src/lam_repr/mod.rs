mod bottom;
mod first;
mod incr;
mod list;
mod pean_chars;
mod second;
mod stdin_list;
mod triple;
mod y_comb;

pub use self::{
    bottom::*, first::*, incr::*, list::*, pean_chars::*, second::*,
    stdin_list::*, triple::*, y_comb::*,
};

/// `\t f.t`
pub type True = First;
/// `\t f.f`
pub type False = Second;
/// `\f x.x`
pub type Zero = Second;
