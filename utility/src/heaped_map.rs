use super::heap_sort;

use super::array::Array;
use super::index::UnsignedInt;

pub struct HeapedMap<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&(SizeT, ValueT), &(SizeT, ValueT)) -> std::cmp::Ordering,
{
    compare: CompareT,
    index_to_position: Array<SizeT, SizeT>,
    item_array: Array<SizeT, (SizeT, ValueT)>,
}

impl<SizeT, ValueT, CompareT> Default for HeapedMap<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&(SizeT, ValueT), &(SizeT, ValueT)) -> std::cmp::Ordering + Default,
{
    fn default() -> Self {
        Self { compare: CompareT::default(), index_to_position: Array::default(), item_array: Array::default() }
    }
}

impl<SizeT, ValueT, CompareT> Clone for HeapedMap<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    ValueT: Clone,
    CompareT: Fn(&(SizeT, ValueT), &(SizeT, ValueT)) -> std::cmp::Ordering + Clone,
{
    fn clone(&self) -> Self {
        Self {
            compare: self.compare.clone(),
            index_to_position: self.index_to_position.clone(),
            item_array: self.item_array.clone(),
        }
    }
}

impl<SizeT, ValueT, CompareT> HeapedMap<SizeT, ValueT, CompareT>
where
    SizeT: UnsignedInt,
    CompareT: Fn(&(SizeT, ValueT), &(SizeT, ValueT)) -> std::cmp::Ordering,
{
    const NULL_POSITION: SizeT = SizeT::MAX;

    pub fn new(compare: CompareT) -> Self {
        Self { compare: compare, index_to_position: Array::default(), item_array: Array::default() }
    }

    // pub fn redimension(&mut self, new_dimension: SizeT) {
    //     if new_dimension > self.index_to_position.len() {
    //         self.index_to_position.resize(new_dimension, Self::NULL_POSITION);
    //     }
    // }

    // #[inline(always)]
    // pub fn dimension(&self) -> SizeT {
    //     self.index_to_position.len()
    // }

    #[inline(always)]
    pub fn len(&self) -> SizeT {
        self.item_array.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.item_array.is_empty()
    }

    #[inline(always)]
    pub fn first(&self) -> Option<(&SizeT, &ValueT)> {
        self.item_array.first().map(|(index, value)| (index, value))
    }

    #[inline(always)]
    pub fn contains_key(&self, index: SizeT) -> bool {
        if index >= self.index_to_position.len() {
            return false;
        } else {
            return self.index_to_position[index] != Self::NULL_POSITION;
        }
    }

    #[inline(always)]
    pub fn get(&self, index: SizeT) -> Option<&ValueT> {
        if index >= self.index_to_position.len() {
            return None;
        } else {
            let position = self.index_to_position[index];
            if position == Self::NULL_POSITION { None } else { Some(&self.item_array[position].1) }
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, index: SizeT, value: ValueT) {
        if index >= self.index_to_position.len() {
            self.index_to_position.resize(index + SizeT::from_usize(1), Self::NULL_POSITION);
        }
        let position = &mut self.index_to_position[index];
        if *position == Self::NULL_POSITION {
            *position = self.item_array.len();
            self.item_array.push((index, value));
            let position = *position;
            heap_sort::up_heap_with_callback(&mut self.item_array, position, &self.compare, |a, b| {
                self.index_to_position.swap(a.0, b.0)
            });
        } else {
            debug_assert!(self.item_array[*position].0 == index);
            self.item_array[*position].1 = value;
            let position = *position;
            heap_sort::update_heap_with_callback(&mut self.item_array, position, &self.compare, |a, b| {
                self.index_to_position.swap(a.0, b.0)
            });
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, index: SizeT) -> Option<ValueT> {
        if index >= self.index_to_position.len() {
            return None;
        } else {
            let position = self.index_to_position[index];
            if position == Self::NULL_POSITION {
                return None;
            } else {
                debug_assert!(self.item_array[position].0 == index);
                let value = self.item_array.swap_remove(position).1;
                self.index_to_position[index] = Self::NULL_POSITION;
                if position != self.item_array.len() {
                    debug_assert!(self.index_to_position[self.item_array[position].0] == self.item_array.len());
                    self.index_to_position[self.item_array[position].0] = position;
                    heap_sort::update_heap_with_callback(&mut self.item_array, position, &self.compare, |a, b| {
                        self.index_to_position.swap(a.0, b.0)
                    });
                };
                return Some(value);
            }
        }
    }

    #[inline(always)]
    pub fn pop_first(&mut self) -> Option<(SizeT, ValueT)> {
        if self.item_array.is_empty() {
            return None;
        } else {
            let (index, value) = self.item_array.swap_remove(SizeT::ZERO);
            debug_assert!(self.index_to_position[index] == SizeT::ZERO);
            self.index_to_position[index] = Self::NULL_POSITION;
            if !self.item_array.is_empty() {
                debug_assert!(self.index_to_position[self.item_array[SizeT::ZERO].0] == self.item_array.len());
                self.index_to_position[self.item_array[SizeT::ZERO].0] = SizeT::ZERO;
                heap_sort::down_heap_with_callback(&mut self.item_array, SizeT::ZERO, &self.compare, |a, b| {
                    self.index_to_position.swap(a.0, b.0)
                });
            };
            return Some((index, value));
        }
    }

    pub fn clear(&mut self) {
        while !self.item_array.is_empty() {
            let index = unsafe { self.item_array.pop().unwrap_unchecked() }.0;
            debug_assert!(self.index_to_position[index] == self.item_array.len());
            self.index_to_position[index] = Self::NULL_POSITION;
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (&SizeT, &ValueT)> + Clone {
        self.item_array.iter().map(|(index, value)| (index, value))
    }
}

impl<IndexT, ValueT, CompareT> std::fmt::Debug for HeapedMap<IndexT, ValueT, CompareT>
where
    IndexT: UnsignedInt,
    ValueT: std::fmt::Debug,
    CompareT: Fn(&(IndexT, ValueT), &(IndexT, ValueT)) -> std::cmp::Ordering,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
