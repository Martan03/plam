use crate::{
    expr::{ExprId, ExprTree},
    lam_repr::{Incr, Zero},
};

/// Encodes numbers as peano.
#[derive(Debug, Clone)]
pub struct PeanoChars(Box<[ExprId; 256]>);

impl PeanoChars {
    /// Create new peano encoder.
    pub fn new(et: &mut ExprTree, incr: &Incr, zero: Zero) -> Self {
        let mut num: ExprId = zero.into();
        let nums = (0..256)
            .map(|_| {
                let res = num.clone();
                num = incr.incr(et, num.clone());
                res
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self(nums)
    }

    /// Get peano encoding for the given number.
    pub fn get(&self, chr: u8) -> ExprId {
        self.0[chr as usize].clone()
    }
}
