use super::{array::Array, heap_sort, index::UnsignedInt};
use std::fmt::Debug;

pub struct PriorityQueue<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
{
    compare: CompareT,
    array: Array<SizeT, ValueT>,
}

impl<SizeT, ValueT, CompareT> PriorityQueue<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
{
    pub fn len(&self) -> SizeT {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn peek(&self) -> Option<&ValueT> {
        self.array.first()
    }

    pub fn reserve(&mut self, additional: SizeT) {
        self.array.reserve(additional);
    }

    pub fn push(&mut self, value: ValueT) {
        let position = self.array.len();
        self.array.push(value);
        heap_sort::up_heap(&mut self.array, position, &self.compare);
    }

    pub fn pop(&mut self) -> Option<ValueT> {
        if self.array.is_empty() {
            return None;
        } else {
            let value = self.array.swap_remove(SizeT::ZERO);
            if !self.array.is_empty() {
                heap_sort::down_heap(&mut self.array, SizeT::ZERO, &self.compare);
            }
            return Some(value);
        }
    }

    pub fn clear(&mut self) {
        self.array.clear();
    }
}

impl<SizeT, ValueT, CompareT> Default for PriorityQueue<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&ValueT, &ValueT) -> std::cmp::Ordering + Default,
{
    fn default() -> Self {
        Self { compare: CompareT::default(), array: Array::default() }
    }
}

impl<SizeT, ValueT, CompareT> Clone for PriorityQueue<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    ValueT: Clone,
    CompareT: Fn(&ValueT, &ValueT) -> std::cmp::Ordering + Clone,
{
    fn clone(&self) -> Self {
        Self { compare: self.compare.clone(), array: self.array.clone() }
    }
}

impl<SizeT, ValueT, CompareT> Debug for PriorityQueue<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&ValueT, &ValueT) -> std::cmp::Ordering + Clone,
    Array<SizeT, ValueT>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.array.fmt(f)
    }
}
