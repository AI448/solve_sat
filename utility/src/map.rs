use super::array::Array;
use super::index::UnsignedInt;

pub struct Map<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    index_to_position: Array<SizeT, SizeT>,
    item_array: Array<SizeT, (SizeT, ValueT)>,
}

impl<SizeT, ValueT> Default for Map<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    fn default() -> Self {
        Self { index_to_position: Array::default(), item_array: Array::default() }
    }
}

impl<SizeT, ValueT> Clone for Map<SizeT, ValueT>
where
    SizeT: UnsignedInt,
    ValueT: Clone,
{
    fn clone(&self) -> Self {
        Self { index_to_position: self.index_to_position.clone(), item_array: self.item_array.clone() }
    }
}

impl<SizeT, ValueT> Map<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    const NULL_POSITION: SizeT = SizeT::MAX;

    #[inline(always)]
    pub fn len(&self) -> SizeT {
        self.item_array.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.item_array.is_empty()
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
        } else {
            debug_assert!(self.item_array[*position].0 == index);
            self.item_array[*position].1 = value;
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
                };
                return Some(value);
            }
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

impl<IndexT, ValueT> std::fmt::Debug for Map<IndexT, ValueT>
where
    IndexT: UnsignedInt,
    ValueT: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
