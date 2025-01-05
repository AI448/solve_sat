use std::{hint::unreachable_unchecked, ops::Not};

use super::boolean::Boolean;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    bits: u32,
}

impl Literal {
    #[inline(always)]
    pub fn new(index: u32, value: Boolean) -> Self {
        debug_assert!(((index << 1) >> 1) == index);
        Self { bits: (index << 1) | value as u32 }
    }

    #[inline(always)]
    pub fn index(&self) -> u32 {
        self.bits >> 1
    }

    #[inline(always)]
    pub fn value(&self) -> Boolean {
        match self.bits & 1 {
            0 => Boolean::FALSE,
            1 => Boolean::TRUE,
            _ => {
                debug_assert!(false);
                unsafe { unreachable_unchecked() }
            }
        }
    }

    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}

impl Not for Literal {
    type Output = Literal;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Literal { bits: self.bits ^ 1 }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", if self.value() == Boolean::FALSE { "!" } else { "" }, self.index())
    }
}
