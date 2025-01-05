use super::literal::Literal;
use std::ops::{Index, IndexMut};
use utility::Array;

pub struct LiteralArray<ValueT> {
    array: Array<u32, ValueT>,
}

impl<ValueT> Default for LiteralArray<ValueT> {
    fn default() -> Self {
        Self { array: Array::default() }
    }
}

impl<ValueT: Clone> Clone for LiteralArray<ValueT> {
    fn clone(&self) -> Self {
        Self { array: self.array.clone() }
    }
}

impl<ValueT> LiteralArray<ValueT> {
    pub fn len(&self) -> u32 {
        self.array.len() / 2
    }

    pub fn push(&mut self, value: [ValueT; 2]) {
        self.array.extend(value);
    }

    pub fn resize_with(&mut self, new_len: u32, f: impl Fn() -> [ValueT; 2]) {
        if self.array.len() < new_len * 2 {
            while self.array.len() < new_len * 2 {
                self.push(f());
            }
        } else if self.array.len() > new_len * 2 {
            self.array.truncate(new_len * 2);
        }
    }
}

impl<ValueT> Index<Literal> for LiteralArray<ValueT> {
    type Output = ValueT;
    fn index(&self, literal: Literal) -> &Self::Output {
        &self.array[literal.bits()]
    }
}

impl<ValueT> IndexMut<Literal> for LiteralArray<ValueT> {
    fn index_mut(&mut self, literal: Literal) -> &mut Self::Output {
        &mut self.array[literal.bits()]
    }
}
