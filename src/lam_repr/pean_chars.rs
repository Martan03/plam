use std::rc::Rc;

use crate::{
    expr::Expr,
    lam_repr::{Incr, Zero},
};

/// Encodes numbers as peano.
#[derive(Debug, Clone)]
pub struct PeanoChars(Box<[Rc<Expr>; 256]>);

impl PeanoChars {
    /// Create new peano encoder.
    pub fn new(incr: &Incr, zero: Zero) -> Self {
        let mut num: Rc<Expr> = zero.into();
        let nums = (0..256)
            .map(|_| {
                let res = num.clone();
                num = incr.incr(num.clone());
                res
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self(nums)
    }

    /// Get peano encoding for the given number.
    pub fn get(&self, chr: u8) -> Rc<Expr> {
        self.0[chr as usize].clone()
    }
}
